package database

import (
	"fmt"
	"os"

	"github.com/jinzhu/gorm"
	// we don't need to use it but its necessary to connect db
	_ "github.com/jinzhu/gorm/dialects/postgres"
	"github.com/joho/godotenv"
)

var db *gorm.DB //database

func init() {
	environ := godotenv.Load()
	if environ != nil {
		fmt.Print(environ)
	}

	username := os.Getenv("db_user")
	password := os.Getenv("db_pass")
	dbname := os.Getenv("db_name")
	dbhost := os.Getenv("db_host")

	dbURI := fmt.Sprintf("host=%s user=%s dbname=%s sslmode=disable password=%s",
		dbhost, username, dbname, password)

	fmt.Println(dbURI)

	conn, err := gorm.Open("postgres", dbURI)

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	db = conn

	// db.Debug().AutoMigrate() // tables

}

// GetDB returns to the database object
func GetDB() *gorm.DB {
	return db
}
