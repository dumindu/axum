CREATE TABLE "books" (
    "id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "published_date" DATE NOT NULL,
    "status" SMALLINT NOT NULL,
    "title" TEXT NOT NULL,
    "image_url" TEXT,
    "description" TEXT,
    PRIMARY KEY ("id")
);
CREATE TABLE "authors" (
    "id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    PRIMARY KEY ("id")
);
