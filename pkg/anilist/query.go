package anilist

// Main query for anilist API.
// FIXME: @perPage and @page static or variable? What was the requirement for this?
var query = `
query ($page: Int = 1, $perPage: Int = 1, $id: Int, $type: MediaType = MANGA) {
	Page(page: $page, perPage: $perPage) {
	  pageInfo {
		total
		perPage
		currentPage
		lastPage
		hasNextPage
	  }
	  media(id: $id, type: $type) {
		id
		idMal
		coverImage {
		  large
		  medium
		}
		bannerImage
		title {
		  romaji
		  english
		  native
		}
		startDate {
		  year
		  month
		  day
		}
		endDate {
		  year
		  month
		  day
		}
		status
		chapters
		volumes
		genres
		tags {
		  name
		  rank
		  category
			  isGeneralSpoiler
		  isMediaSpoiler
		}
		popularity
		staff {
		  edges {
			id
			role
			node{
			  name {
				first
				last
				native
			  }
			  image {
				large
				medium
			  }
			}
		  }
		}
		characters {
		  edges {
			id
			role
			node{
			  image {
				large
				medium
			  }
			  name {
				first
				last
				native
			  }
			}
		  }
		}
		relations {
		  edges {
			id
			  relationType
			node{
			  bannerImage
			  title {
				romaji
				english
				native
			  }
			  type
			  status
			  idMal
			}
		  }
		}
	  }
	}
  }
  
`
