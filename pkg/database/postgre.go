package database

import (
	"fmt"
	"log"
	"os"

	"github.com/jinzhu/gorm"
	// we don't need to use it but its necessary to connect db
	_ "github.com/jinzhu/gorm/dialects/postgres"
	"github.com/joho/godotenv"
	uuid "github.com/satori/go.uuid"
)

var db *gorm.DB //database

type manga struct {
	gorm.Model
	MangaID     uuid.UUID `gorm:"type:uuid;primary_key:true;"`
	CoverImage  string    `gorm:"type:text;"`
	BannerImage string    `gorm:"type:text;"`
	StartDate   string    `gorm:"type:text;"`
	EndDate     string    `gorm:"type:text;"`
	Status      string    `gorm:"type:text;"`
	Staff       []staff   `gorm:"many2many:Series;association_foreignkey:AuthorID;foreignkey:MangaID"`
}

type staff struct {
	gorm.Model
	AuthorID uuid.UUID `gorm:"type:uuid;primary_key:true;"`
	Role     string    `gorm:"type:text;"`
	Name     string    `gorm:"type:text;"`
	Image    string    `gorm:"type:text;"`
}

// Initialize ..
func Initialize() {
	environ := godotenv.Load()
	if environ != nil {
		log.Fatalln(environ)
	}

	username := os.Getenv("db_user")
	password := os.Getenv("db_pass")
	dbname := os.Getenv("db_name")
	dbhost := os.Getenv("db_host")

	dbURI := fmt.Sprintf("host=%s user=%s dbname=%s sslmode=disable password=%s",
		dbhost, username, dbname, password)

	log.Println(dbURI)

	conn, err := gorm.Open("postgres", dbURI)

	if err != nil {
		log.Fatalln(err)
		os.Exit(1)
	}

	db = conn

	db.Debug().AutoMigrate(&manga{}, &staff{}) // tables
}

// GetDB returns to the database object
func GetDB() *gorm.DB {
	return db
}
