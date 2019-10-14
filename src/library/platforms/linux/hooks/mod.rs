/*
exec hooks: Required
+--------+-------------------------------------------------------------------------------------+
| Letter |                                       Meaning                                       |
+--------+-------------------------------------------------------------------------------------+
| e      | Takes an extra argument to provide the environment of the new program               |
| l      | Takes the arguments of the new program as a variable-length argument list           |
| p      | Searches the PATH environment variable to find the program if a path isn't provided |
| v      | Takes an array parameter to specify the argv[] array of the new program             |
+--------+-------------------------------------------------------------------------------------+
*/

// TODO: Limited support for variadic functions? (l)
mod execl;
mod execle;
mod execlp;
mod execv;
mod execve;
mod execvp;
mod execvpe;
mod fexecve;

/*
TODO: open hooks: Optional
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
attr/acl hooks for various filesystems, *init_module, chroot: Optional
*/
