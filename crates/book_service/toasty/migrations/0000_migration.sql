CREATE TABLE "books" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "id" UUID NOT NULL,
    "published_date" DATE NOT NULL,
    "status" SMALLINT NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "image_url" TEXT,
    PRIMARY KEY ("id")
);

CREATE TABLE "authors" (
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    PRIMARY KEY ("id")
);
