BEGIN;

-- TODO: Essential hooks, rules

INSERT INTO LogClass (class) VALUES ("None"),("Error"),("Auth"),("General");

INSERT INTO Setting (param, value) VALUES ("ConsoleSecret", "undefined"),
                                          ("ConsoleSecretExpiry", "-1"),
                                          ("HashAlgorithm", "BLAKE3"),
                                          ("LogVerbosity", CAST((SELECT id FROM LogClass WHERE class="General") AS TEXT)),
                                          ("Prevention", "false"),
                                          ("RecoverySecret", "undefined"),
                                          ("RotateLogLimit", "10000"),
                                          ("RotateNonceLimit", "3600"),
                                          ("SecretAlgorithm", "ARGON2ID"),
                                          ("ServerIP", "undefined"),
                                          ("ServerPublicKey", "undefined"),
                                          ("ServerType", "undefined"),
                                          ("ServicePort", "11998"),
                                          ("SettingsModified", "-1");

COMMIT;
