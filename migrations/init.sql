CREATE TABLE IF NOT EXISTS "device_type" (
	"id"	INTEGER,
	"hard_version"	TEXT NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	"category"	INTEGER NOT NULL,
	"has_ble"	INTEGER NOT NULL DEFAULT 1,
	"has_finger"	INTEGER NOT NULL DEFAULT 1,
	"has_stm32"	INTEGER NOT NULL DEFAULT 0,
	"desc"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "firm" (
	"id"	INTEGER,
	"hard_version"	INTEGER NOT NULL,
	"version_name"	TEXT NOT NULL,
	"version_format"	TEXT,
	"version_type"	INTEGER NOT NULL,
	"finger_level"	INTEGER NOT NULL DEFAULT 0,
	"url"	INTEGER NOT NULL,
	"desc"	TEXT NOT NULL DEFAULT '',
	"update_time"	datetime DEFAULT current_timestamp,
	"rely_version_type"	INTEGER,
	"min"	TEXT,
	"max"	TEXT,
	"des_en"	TEXT DEFAULT '',
	"des_ko"	TEXT DEFAULT '',
	"des_sp"	TEXT DEFAULT '',
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "user" (
	"id"	INTEGER,
	"name"	TEXT,
	"mail"	TEXT UNIQUE,
	"password"	TEXT,
	"update_time"	datetime DEFAULT current_timestamp,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS "version_type" (
	"id"	INTEGER,
	"name"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);