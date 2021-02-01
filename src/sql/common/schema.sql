BEGIN;

-- TODO: Text table for distinguishing Essential/Base/etc.?

CREATE TABLE LogClass (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  class TEXT NOT NULL,
  UNIQUE(class)
);

CREATE TABLE Log (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  class INTEGER NOT NULL,
  desc TEXT,
  ts INTEGER NOT NULL,
  FOREIGN KEY (class) REFERENCES LogClass (id)
);

CREATE TABLE WhitelistClass (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  class TEXT NOT NULL,
  UNIQUE(class)
);

CREATE TABLE Whitelist (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  path TEXT NOT NULL,
  value TEXT NOT NULL,
  class INTEGER NOT NULL,
  UNIQUE(path, value, class),
  FOREIGN KEY (class) REFERENCES WhitelistClass (id)
);

CREATE TABLE Action (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL,
  UNIQUE(name)
);

CREATE TABLE Setting (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  param TEXT NOT NULL,
  value TEXT NOT NULL,
  UNIQUE(param)
);

CREATE INDEX SettingIndex ON Setting (param, value);

CREATE TABLE NonceHistory (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  nonce TEXT NOT NULL,
  ts INTEGER NOT NULL,
  UNIQUE(nonce)
);

CREATE INDEX NonceHistoryIndex ON NonceHistory (nonce, ts);

CREATE TABLE HookClass (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  class TEXT NOT NULL,
  UNIQUE(class)
);

CREATE TABLE HookLanguage (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  language TEXT NOT NULL,
  UNIQUE(language)
);

CREATE TABLE Hook (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  symbol TEXT NOT NULL,
  library TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 0,
  language INTEGER NOT NULL,
  class INTEGER,
  UNIQUE(symbol, library),
  FOREIGN KEY (language) REFERENCES HookLanguage (id),
  FOREIGN KEY (class) REFERENCES HookClass (id)
);

CREATE TABLE Datatype (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  datatype TEXT NOT NULL,
  pointer INTEGER NOT NULL DEFAULT 0,
  signed INTEGER NOT NULL DEFAULT 1,
  variadic INTEGER NOT NULL DEFAULT 0,
  array INTEGER NOT NULL DEFAULT 0,
  UNIQUE(datatype)
);

CREATE TABLE Argument (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL,
  parent INTEGER,
  position INTEGER NOT NULL,
  hook INTEGER NOT NULL,
  datatype INTEGER NOT NULL,
  UNIQUE(name, parent, hook),
  UNIQUE(position, parent, hook),
  FOREIGN KEY (parent) REFERENCES Argument (id),
  FOREIGN KEY (hook) REFERENCES Hook (id),
  FOREIGN KEY (datatype) REFERENCES Datatype (id)
);

CREATE INDEX ArgumentIndex ON Argument (parent);

CREATE TABLE Rule (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  arg INTEGER NOT NULL,
  action INTEGER NOT NULL,
  UNIQUE(arg, action),
  FOREIGN KEY (arg) REFERENCES Argument (id),
  FOREIGN KEY (action) REFERENCES Action (id)
);

CREATE TRIGGER RotateLog AFTER INSERT ON Log
  BEGIN
    -- Rotate logs
    DELETE FROM Log WHERE ts=(SELECT min(ts) FROM Log) AND (SELECT count(*) FROM Log)=CAST((SELECT value FROM Setting WHERE param='RotateLogLimit') AS INTEGER);
  END;

CREATE TRIGGER RotateNonceHistory AFTER INSERT ON NonceHistory
  BEGIN
    -- Delete expired nonces
    DELETE FROM NonceHistory WHERE ts < strftime('%s','now')-CAST((SELECT value FROM Setting WHERE param='RotateNonceLimit') AS INTEGER);
  END;

CREATE VIEW HookView AS
     SELECT HookLanguage.language,
            Hook.library,
            Hook.symbol,
            Hook.id
       FROM Hook
       INNER JOIN HookLanguage ON Hook.language = HookLanguage.id
       WHERE Hook.enabled = TRUE;

CREATE VIEW ArgumentView AS
     SELECT Argument.hook,
            Argument.parent,
            Argument.id,
            Argument.position,
            Datatype.datatype,
            Datatype.pointer,
            Datatype.signed,
            Datatype.variadic,
            Datatype.array
       FROM Argument
       INNER JOIN Hook ON Argument.hook = Hook.id
       INNER JOIN HookLanguage ON Hook.language = HookLanguage.id
       INNER JOIN Datatype ON Argument.datatype = Datatype.id
       WHERE Hook.enabled = TRUE;

CREATE VIEW WhitelistView AS
     SELECT WhitelistClass.class,
            Whitelist.path,
            Whitelist.value
       FROM Whitelist
       INNER JOIN WhitelistClass ON Whitelist.class = WhitelistClass.id;

CREATE VIEW RuleView AS
     SELECT Rule.arg,
            Action.name AS action
       FROM Rule
       INNER JOIN Action ON Rule.action = Action.id;

COMMIT;
