use std::ffi::c_void;
use std::path::PathBuf;
use std::process::Command;

use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::{Com::*, LibraryLoader::*},
        UI::Shell::*,
    },
};

// Le numéro d'identité de l'employé (ton GUID)
#[allow(dead_code)]
const CLSID_NATIVECONVERT: GUID = GUID::from_u128(0x625d08ba_123b_4441_a87b_78363629e7a9);

// Où se trouve la DLL ? (l'exe et le logo sont rangés à côté d'elle)
fn dossier_de_la_dll() -> Option<PathBuf> {
    unsafe {
        let mut module = HMODULE::default();
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            PCWSTR(dossier_de_la_dll as *const () as *const u16),
            &mut module,
        )
        .ok()?;

        let mut tampon = [0u16; 1024];
        let longueur = GetModuleFileNameW(Some(module), &mut tampon) as usize;
        if longueur == 0 {
            return None;
        }
        let chemin = PathBuf::from(String::from_utf16_lossy(&tampon[..longueur]));
        chemin.parent().map(|p| p.to_path_buf())
    }
}

// L'employé : un objet COM qui sait faire les gestes d'IExplorerCommand
#[implement(IExplorerCommand)]
struct CommandeNativeConvert;

impl IExplorerCommand_Impl for CommandeNativeConvert_Impl {
    // Le texte affiché dans le menu
    fn GetTitle(&self, _elements: Ref<'_, IShellItemArray>) -> Result<PWSTR> {
        unsafe { SHStrDupW(w!("NativeConvert")) }
    }

    // L'icône : le logo.ico rangé à côté de la DLL
    fn GetIcon(&self, _elements: Ref<'_, IShellItemArray>) -> Result<PWSTR> {
        let dossier = dossier_de_la_dll().ok_or_else(|| Error::from(E_FAIL))?;
        let icone = dossier.join("logo.ico");
        let texte = HSTRING::from(icone.to_string_lossy().as_ref());
        unsafe { SHStrDupW(PCWSTR(texte.as_ptr())) }
    }

    // L'info-bulle : pas besoin
    fn GetToolTip(&self, _elements: Ref<'_, IShellItemArray>) -> Result<PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetCanonicalName(&self) -> Result<GUID> {
        Err(E_NOTIMPL.into())
    }

    // Le bouton est actif (0 = ECS_ENABLED)
    fn GetState(&self, _elements: Ref<'_, IShellItemArray>, _lent_ok: BOOL) -> Result<u32> {
        Ok(0)
    }

    // Ce qui se passe au clic : lancer NativeConvert.exe avec les chemins sélectionnés
    fn Invoke(
        &self,
        elements: Ref<'_, IShellItemArray>,
        _contexte: Ref<'_, IBindCtx>,
    ) -> Result<()> {
        let elements = elements.ok()?;
        let dossier = dossier_de_la_dll().ok_or_else(|| Error::from(E_FAIL))?;
        let exe = dossier.join("NativeConvert.exe");

        let mut chemins: Vec<String> = Vec::new();
        unsafe {
            let nombre = elements.GetCount()?;
            for i in 0..nombre {
                let element = elements.GetItemAt(i)?;
                let nom = element.GetDisplayName(SIGDN_FILESYSPATH)?;
                let texte = nom.to_string().map_err(|_| Error::from(E_FAIL))?;
                chemins.push(texte);
                CoTaskMemFree(Some(nom.0 as *const c_void));
            }
        }

        Command::new(exe)
            .args(chemins)
            .spawn()
            .map_err(|_| Error::from(E_FAIL))?;
        Ok(())
    }

    // Comportement par défaut (0 = ECF_DEFAULT)
    fn GetFlags(&self) -> Result<u32> {
        Ok(0)
    }

    // Pas de sous-menu
    fn EnumSubCommands(&self) -> Result<IEnumExplorerCommand> {
        Err(E_NOTIMPL.into())
    }
}

// L'usine : son seul travail est de fabriquer l'employé
#[implement(IClassFactory)]
struct Usine;

impl IClassFactory_Impl for Usine_Impl {
    fn CreateInstance(
        &self,
        _parent: Ref<'_, IUnknown>,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> Result<()> {
        unsafe {
            let employe: IExplorerCommand = CommandeNativeConvert.into();
            employe.query(riid, ppv).ok()
        }
    }

    fn LockServer(&self, _verrou: BOOL) -> Result<()> {
        Ok(())
    }
}

// La porte d'entrée : l'Explorateur demande "j'ai besoin de l'objet <CLSID>"
#[unsafe(no_mangle)]
pub extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    unsafe {
        if ppv.is_null() {
            return E_POINTER;
        }
        *ppv = std::ptr::null_mut();

        if *rclsid != CLSID_NATIVECONVERT {
            return CLASS_E_CLASSNOTAVAILABLE;
        }

        let usine: IClassFactory = Usine.into();
        usine.query(riid, ppv)
    }
}


#[unsafe(no_mangle)]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_FALSE
}