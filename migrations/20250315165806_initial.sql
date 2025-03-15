CREATE TABLE "Company" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "AppUser" (
	"id"	INTEGER,
	"username"	TEXT NOT NULL UNIQUE,
	"email"	TEXT,
	"name"	TEXT NOT NULL,
	"password"	TEXT NOT NULL,
	"company_id"	INTEGER NOT NULL,
	FOREIGN KEY("company_id") REFERENCES "Company"("id"),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE INDEX "idx_AppUser_company_id" ON "AppUser" ("company_id");

CREATE TABLE "Payroll" (
	"id"	INTEGER,
	"date"	TEXT NOT NULL,
	"user_id"	INTEGER NOT NULL,
	"object_key"	TEXT NOT NULL UNIQUE,
	"filename"	TEXT NOT NULL,
	"content_type"	TEXT NOT NULL,
	"file_size"	INTEGER NOT NULL,
	"uploaded_at"	TEXT NOT NULL,
	FOREIGN KEY("user_id") REFERENCES "AppUser"("id"),
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE INDEX "idx_Payroll_user_id" ON "Payroll" ("user_id");

CREATE TABLE "Permission" (
	"user_id"	INTEGER,
	"user"	INTEGER NOT NULL,
	"payroll"	INTEGER NOT NULL,
	"company"	INTEGER NOT NULL,
	PRIMARY KEY("user_id"),
	FOREIGN KEY("user_id") REFERENCES "AppUser"("id") ON DELETE CASCADE
);

INSERT INTO "Company" ("name") VALUES ("SuperAdminCompany");
INSERT INTO "AppUser" ("username", "name", "password", "company_id") VALUES ("super.admin", "Super Admin", "$2a$12$iQzXMMzW77eEvjRw5GZ57u3i4gSTlUE2AMk9vsDe2wcq7p8SGBF7m", 1);
INSERT INTO "Permission" ("user_id", "user", "payroll", "company") VALUES (1, 15, 15, 15);
