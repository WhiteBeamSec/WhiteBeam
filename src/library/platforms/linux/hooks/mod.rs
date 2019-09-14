/*
exec hooks
*/
// Primary hook
mod execve;

// Secondary hooks
mod execle;
mod execvpe;
mod fexecve;
mod execl;
mod execlp;
mod execv;
mod execvp;

/*
TODO: Protect mem and other system files using mode of open
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
// TODO: are fd functions necessary? They depend on above.
