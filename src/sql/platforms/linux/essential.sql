BEGIN;

/*
Title: Essential
Description: Minimum hooks, rules, and whitelist entries required to protect and run WhiteBeam
Publisher: WhiteBeam Security, Inc.
Version: 0.2 Alpha
*/

-- Whitelist: Libraries will go here too
INSERT INTO Whitelist (path, value, class) VALUES ("ANY", "/bin/bash", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/bin/sh", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/usr/bin/bash", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/usr/bin/sh", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/opt/WhiteBeam/whitebeam", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("/opt/WhiteBeam/whitebeam", "11998", (SELECT id FROM WhitelistClass WHERE class="Network/Range/Port"));

-- Hook
INSERT INTO Hook (symbol, library, enabled, language, class) VALUES ("execl", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execle", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execlp", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execv", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execve", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execvp", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execvpe", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("fexecve", "/lib/x86_64-linux-gnu/libc.so.6", 0, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution"));

-- Argument
INSERT INTO Argument (name, position, hook, datatype) VALUES -- execl
                                                             ("pathname", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execl"), (SELECT id FROM Datatype WHERE datatype="String")),
                                                             ("arg", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execl"), (SELECT id FROM Datatype WHERE datatype="StringVariadic")),
                                                             -- execle
                                                             ("pathname", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execle"), (SELECT id FROM Datatype WHERE datatype="String")),
                                                             ("arg", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execle"), (SELECT id FROM Datatype WHERE datatype="StringVariadic")),
                                                             ("envp", 2, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execle"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             -- execlp
                                                             ("file", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execlp"), (SELECT id FROM Datatype WHERE datatype="String")),
                                                             ("arg", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execlp"), (SELECT id FROM Datatype WHERE datatype="StringVariadic")),
                                                             -- execv
                                                             ("pathname", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execv"), (SELECT id FROM Datatype WHERE datatype="String")),
                                                             ("argv", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execv"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             -- execve
                                                             ("filename", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execve"), (SELECT id FROM Datatype WHERE datatype="String")),
                                                             ("argv", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execve"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             ("envp", 2, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execve"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             -- execvp
                                                             ("file", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvp"), (SELECT id FROM Datatype WHERE datatype="String")),
                                                             ("argv", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvp"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             -- execvpe
                                                             ("file", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvpe"), (SELECT id FROM Datatype WHERE datatype="String")),
                                                             ("argv", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvpe"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             ("envp", 2, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvpe"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             -- fexecve
                                                             ("fd", 0, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="fexecve"), (SELECT id FROM Datatype WHERE datatype="IntegerSigned")),
                                                             ("argv", 1, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="fexecve"), (SELECT id FROM Datatype WHERE datatype="StringArray")),
                                                             ("envp", 2, (SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="fexecve"), (SELECT id FROM Datatype WHERE datatype="StringArray"));

-- Rule
-- TODO: Audit non-exec*e functions for whitelist bypasses
-- TODO: fexecve
INSERT INTO Rule (arg, action) VALUES -- Execution
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execl") AND name="pathname"), (SELECT id FROM Action WHERE name="VerifyCanExecute")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execl") AND name="pathname"), (SELECT id FROM Action WHERE name="VerifyFileHash")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execle") AND name="envp"), (SELECT id FROM Action WHERE name="FilterEnvironment")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execle") AND name="pathname"), (SELECT id FROM Action WHERE name="VerifyCanExecute")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execle") AND name="pathname"), (SELECT id FROM Action WHERE name="VerifyFileHash")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execlp") AND name="file"), (SELECT id FROM Action WHERE name="VerifyCanExecute")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execlp") AND name="file"), (SELECT id FROM Action WHERE name="VerifyFileHash")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execv") AND name="pathname"), (SELECT id FROM Action WHERE name="VerifyCanExecute")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execv") AND name="pathname"), (SELECT id FROM Action WHERE name="VerifyFileHash")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execve") AND name="envp"), (SELECT id FROM Action WHERE name="FilterEnvironment")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execve") AND name="filename"), (SELECT id FROM Action WHERE name="VerifyCanExecute")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execve") AND name="filename"), (SELECT id FROM Action WHERE name="VerifyFileHash")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvp") AND name="file"), (SELECT id FROM Action WHERE name="VerifyCanExecute")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvp") AND name="file"), (SELECT id FROM Action WHERE name="VerifyFileHash")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvpe") AND name="envp"), (SELECT id FROM Action WHERE name="FilterEnvironment")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvpe") AND name="file"), (SELECT id FROM Action WHERE name="VerifyCanExecute")),
                                      ((SELECT id FROM Argument WHERE hook=(SELECT id FROM Hook WHERE library = "/lib/x86_64-linux-gnu/libc.so.6" AND symbol="execvpe") AND name="file"), (SELECT id FROM Action WHERE name="VerifyFileHash"));

COMMIT;
