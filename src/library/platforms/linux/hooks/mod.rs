// Only the exec hooks are needed to safely run WhiteBeam for most unprivileged services
// The rest are for protecting WhiteBeam against root

/*
exec hooks
*/
// Primary hook
mod execve;

// Secondary hooks
mod execl;
mod execlp;
mod execle;
mod execv;
mod execvp;
mod execvpe;
mod fexecve;

/*
TODO: open hooks
Protect mem, disk, and other system files using open mode
O_RDWR or O_WRONLY is prohibited, including implicitly (creat)
*/
// mod creat
// mod creat64
// mod fopen
// mod fopen64
// mod freopen
// mod freopen64
// mod open
// mod open64
// mod openat
// mod openat64
// mod open_by_handle_at

/*
TODO: (sym)link/unlink*, *chmod*, rename*, makedev/makenod*, mount,
attr/acl hooks for various filesystems, *init_module, chroot
*/
