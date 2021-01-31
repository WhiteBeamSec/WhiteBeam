BEGIN;

-- LogClass: id is equal to the verbosity
-- INSERT INTO LogClass (class) VALUES ("None"),("Error"),("Auth"),("General");

-- Log
INSERT INTO Log (class, desc, ts) VALUES ((SELECT id FROM LogClass WHERE class="Error"), "Fatal error in crypto.rs line 32: Unhandled exception", 1590000000),
                                         ((SELECT id FROM LogClass WHERE class="Error"), "Fatal error in crypto.rs line 51: Unhandled exception", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="Auth"), "User root successfully authenticated to WhiteBeam", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="Auth"), "User root failed to authenticate to WhiteBeam", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="Auth"), "User nobody failed to authenticate to WhiteBeam", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="General"), "Received request for public key from 172.16.0.2", strftime("%s", "now"));

-- WhitelistClass
INSERT INTO WhitelistClass (class) VALUES ("Hash/SHA3-256"),
                                          ("Hash/SHA3-512"),
                                          ("Hash/BLAKE3"),
                                          ("Filesystem/Path/Readable"),
                                          ("Filesystem/Path/Writable"),
                                          ("Filesystem/Path/Executable"),
                                          ("Network/Range/CIDR"),
                                          ("Network/Range/Port"),
                                          ("Certificate/DER"),
                                          ("Certificate/PEM");

-- Whitelist: Libraries will go here too
INSERT INTO Whitelist (path, value, class) VALUES ("/bin/bash", "3599edef28afa67b9bec983d57416d9a2cc33a166527c3f6ce2aabef96f66c52", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/bin/sh", "74704b4c3477ac155c2ca3ebbeb8f10db2badac161e331d006af5820f0acca7a", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/usr/sbin/apache2", "4aadc76a6af5d65197cb9cdf7d7a6945772539c48c0120919f38f77af29c0f53", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/usr/bin/whoami", "758fd29bc9160ab6e302be8c6dae03d2854cceaa5ed1aca525ab57a740a90645", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/bin/bash", "/usr/sbin/apache2", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("/bin/sh", "/usr/sbin/apache2", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/bin/bash", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/bin/sh", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/usr/bin/bash", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/usr/bin/sh", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/usr/bin/whoami", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/tmp/*", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Writable")), -- Realpath/canonicalized open() for wildcards?
                                                  ("/opt/WhiteBeam/whitebeam", "11998", (SELECT id FROM WhitelistClass WHERE class="Network/Range/Port")),
                                                  ("/usr/sbin/apache2", "172.16.0.0/12", (SELECT id FROM WhitelistClass WHERE class="Network/Range/CIDR"));

-- Action
INSERT INTO Action (name) VALUES ("VerifyCanExecute"),("VerifyFileHash"),("FilterEnvironment"),("ConsumeVariadic");

-- NonceHistory
INSERT INTO NonceHistory (nonce, ts) VALUES (lower(hex(randomblob(24))), 1590000000),
                                            (lower(hex(randomblob(24))), strftime("%s", "now")),
                                            (lower(hex(randomblob(24))), strftime("%s", "now")),
                                            (lower(hex(randomblob(24))), strftime("%s", "now"));

-- HookClass
INSERT INTO HookClass (class) VALUES ("Execution"),("MemoryProtection"),("Network"),("Filesystem"),("Certificate"),("Bruteforce");

-- HookLanguage
INSERT INTO HookLanguage (language) VALUES ("C"),("Java"),("PHP"),("Python"),("Ruby");

-- Hook
INSERT INTO Hook (symbol, library, enabled, language, class) VALUES ("execl", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execle", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execlp", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execv", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execve", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execvp", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("execvpe", "/lib/x86_64-linux-gnu/libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution")),
                                                                    ("fexecve", "/lib/x86_64-linux-gnu/libc.so.6", 0, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Execution"));

-- Datatype: 32/64 bit determined with usize
INSERT INTO Datatype (datatype, pointer, signed, variadic, array) VALUES ("String", 1, 0, 0, 0),
                                                                         ("StringArray", 1, 0, 0, 1),
                                                                         ("StringVariadic", 0, 0, 1, 0),
                                                                         ("IntegerSigned", 0, 1, 0, 0),
                                                                         ("IntegerUnsigned", 0, 0, 0, 0),
                                                                         ("Struct", 0, 0, 0, 0),
                                                                         ("StructPointer", 1, 0, 0, 0);

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
