package main

import (
	"fmt"
	"log"
	"os"
	"time"

	"github.com/jinzhu/gorm"
	// we don't need to use it but its necessary to connect db
	_ "github.com/jinzhu/gorm/dialects/postgres"
	"github.com/joho/godotenv"
	uuid "github.com/satori/go.uuid"
)

var db *gorm.DB //database

type Base struct {
	ID        uuid.UUID `gorm:"type:uuid;primary_key;"`
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt *time.Time `sql:"index"`
}

// Manga is the main media resource
type Manga struct {
	Base
	CoverImage    string
	BannerImage   string
	StartDate     string
	EndDate       string
	Status        string
	Staff         []Staff `gorm:"many2many:series;"`
	Title         string
	Relationships []Relations
}

// Staff is the staff of the Media. This could be either author or artist
type Staff struct {
	Base
	Role  string
	Name  string
	Image string
}

// Relations are the media that manga has relationship with. Like anime adaptation and such.
type Relations struct {
	Base
	BannerImage      string
	Title            string
	Type             string
	Status           string
	RelationshipType string
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

	// db.LogMode(true)
	// db.Debug().AutoMigrate(&User{}, &Profile{}) // tables
}

// GetDB returns to the database object
func GetDB() *gorm.DB {
	return db
}

// AddStaff adds new staff
func AddStaff(db *gorm.DB, manga *Manga, staff *Staff) error {
	res := db.Model(manga).Association("Staff").Append(staff)
	return res.Error
}

func CreateManga(db *gorm.DB, manga *Manga) (uuid.UUID, error) {
	res := db.Create(manga)
	if res.Error != nil {
		uid, _ := uuid.FromString("")
		return uid, res.Error
	}
	return manga.ID, nil
}

func CreateStaffIfNotExists(db *gorm.DB, staffName string) (*Staff, error) {
	var stf Staff
	res := db.FirstOrCreate(&stf, Staff{Name: staffName})
	if res.Error != nil {
		return nil, res.Error
	}
	return &stf, nil
}

func NewManga(db *gorm.DB, manga *Manga) (*Manga, error) {
	m := &Manga{
		BannerImage: manga.BannerImage,
		CoverImage:  manga.CoverImage,
		Title:       manga.Title,
		Status:      manga.Status,
		EndDate:     manga.EndDate,
		StartDate:   manga.StartDate,
	}
	tx := db.Begin()
	if tx.Error != nil {
		return nil, tx.Error
	}

	_, err := CreateManga(tx, m)
	if err != nil {
		return nil, err
	}

	for _, stf := range manga.Staff {
		s, err := CreateStaffIfNotExists(tx, stf.Name)
		if err != nil {
			tx.Rollback()
			return nil, err
		}
		err = AddStaff(tx, m, s)
		if err != nil {
			tx.Rollback()
			return nil, err
		}
	}

	res := tx.Commit()
	if res.Error != nil {
		return nil, res.Error
	}
	return &Manga{Title: manga.Title}, nil
}

func main() {
	Initialize()
	manga := Manga{
		CoverImage:  "https://someothershit",
		BannerImage: "https://someothershitothershit",
		Title:       "Oh! Shit",
		Status:      "On going",
		EndDate:     "-",
		StartDate:   "20/01/2015",
		Staff: []Staff{
			{
				Name:  "Ali",
				Role:  "Author",
				Image: "https://someshit.com",
			},
		},
		Relationships: []Relations{
			{
				RelationshipType: "Adaptation",
				Type:             "Anime",
				BannerImage:      "https://uggh",
				Status:           "Finished",
				Title:            "Some shtty",
			},
		},
	}
	_, e := NewManga(db, &manga)
	if e != nil {
		log.Fatalln(e)
	}

}
