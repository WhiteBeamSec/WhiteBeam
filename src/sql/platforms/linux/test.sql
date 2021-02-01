BEGIN;

-- Log
INSERT INTO Log (class, desc, ts) VALUES ((SELECT id FROM LogClass WHERE class="Error"), "Fatal error in crypto.rs line 32: Unhandled exception", 1590000000),
                                         ((SELECT id FROM LogClass WHERE class="Error"), "Fatal error in crypto.rs line 51: Unhandled exception", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="Auth"), "User root successfully authenticated to WhiteBeam", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="Auth"), "User root failed to authenticate to WhiteBeam", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="Auth"), "User nobody failed to authenticate to WhiteBeam", strftime("%s", "now")),
                                         ((SELECT id FROM LogClass WHERE class="General"), "Received request for public key from 172.16.0.2", strftime("%s", "now"));

-- Whitelist: Libraries will go here too
INSERT INTO Whitelist (path, value, class) VALUES ("/bin/bash", "3599edef28afa67b9bec983d57416d9a2cc33a166527c3f6ce2aabef96f66c52", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/bin/sh", "74704b4c3477ac155c2ca3ebbeb8f10db2badac161e331d006af5820f0acca7a", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/usr/sbin/apache2", "4aadc76a6af5d65197cb9cdf7d7a6945772539c48c0120919f38f77af29c0f53", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/usr/bin/whoami", "758fd29bc9160ab6e302be8c6dae03d2854cceaa5ed1aca525ab57a740a90645", (SELECT id FROM WhitelistClass WHERE class="Hash/BLAKE3")),
                                                  ("/bin/bash", "/usr/sbin/apache2", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("/bin/sh", "/usr/sbin/apache2", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/usr/bin/whoami", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Executable")),
                                                  ("ANY", "/tmp/*", (SELECT id FROM WhitelistClass WHERE class="Filesystem/Path/Writable")), -- Realpath/canonicalized open() for wildcards?
                                                  ("/usr/sbin/apache2", "172.16.0.0/12", (SELECT id FROM WhitelistClass WHERE class="Network/Range/CIDR"));

-- NonceHistory
INSERT INTO NonceHistory (nonce, ts) VALUES (lower(hex(randomblob(24))), 1590000000),
                                            (lower(hex(randomblob(24))), strftime("%s", "now")),
                                            (lower(hex(randomblob(24))), strftime("%s", "now")),
                                            (lower(hex(randomblob(24))), strftime("%s", "now"));

COMMIT;
