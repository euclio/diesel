use syn::{Ident, Path, Ty};

pub fn ty_ident(ident: Ident) -> Ty {
    ty_path(ident.into())
}

pub fn ty_path(path: Path) -> Ty {
    Ty::Path(None, path)
}
