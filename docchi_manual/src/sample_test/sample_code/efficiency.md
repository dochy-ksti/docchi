## 1. Efficiency

The Dochy language is basically JSON5 which is a better JSON. JSON5 is dynamically typed, like this.

```json5
{
  list : [
	{
		age: 1,
		name: "dochy1"
	},
	{
		age: 2,
		name: "dochy2"
	}
  ]
}
```
Dynamically typed data must contain type data, namely member names and member types,
which can vary in each item so every item must have them.

In Dochy, member names and types are statically defined and can't vary in a list.
```json5
{
  // Comments can be written in JSON5 (and Dochy)
  list : [ 
    // "MList" means a mutable list, one of the collection types in Dochy.
    // Every collection in Dochy must have the collection type as the first item
    "MList",
    // Every collection in Dochy must have Default Object written within "[{" and "}]".
    // Default Object defines the static type of the items in the list.
    [{
      age : 0, //The type is Int, which is distinguished from Float
      name : "" //String type
    }],
    {
      age : 1,
      name : "dochy1"
    },
    {
      age : 2,
      name : "dochy2",
    }
  ]
}
```
The default object defines the static type, and the type can't vary, 
so we can omit the type data from the items.

Actually, Dochy's data is binary, 
but the system records the difference from the initial state which is described in JSON5.
We need raw JSON5 to define the initial state,

JSON5 is easy to read, write and maintain. 
Real world applications need the initial data for the most time,
so we think that initial raw JSON5 being mandatory is not a serious disadvantage.

We introduce an example to show how Dochy is efficient .
```json5
{
	users : [
		{
			id : 1,
			name : "Holt Gianuzzi",
			created : "2/24/2021",
			is_active : false,
			thumbnail : null
		},
		{
			id : 2,
			name : "Sansone Gerard",
			created : "5/28/2020",
			is_active : true,
			thumbnail : null
		},
		{
			id : 3,
			name : "Max Mangan",
			created : "3/14/2020",
			is_active : false,
			thumbnail : null
		},
		{
			id : 4,
			name : "Brice Bartosch",
			created : "11/24/2020",
			is_active : false,
			thumbnail : null
		},
		{
			id : 5,
			name : "Lauretta Tyt",
			created : "4/26/2020",
			is_active : true,
			thumbnail : null
		}
	]
}
```
This is mock JSON5 data. 

To convert the data to Dochy, at first, we need to write the initial JSON5.
```json5
{
  users: [  
    "MList",
    [{
      id: 0,
      name: "",
      created: "",
      is_active: false,
      // member name with "?" means it's nullable, which is Dochy's extension from JSON5
      thumbnail? : ["Str", null], //null values need the type specified
    }]
  ]
}
```
This is an artificial example. Normally we need to write actual, concrete data for expressiveness.
However, we need to show the data size of Dochy, so the values are emptied.

We made the data inputted and output the string expression of the equivalent Dochy data. 
```json5
{
	"Old" : [], // Old is a member added by the system.
	"users" : [
		"MList",
		[ "NextID", 5 ],
		[ {
			"created" : "",
			"id" : 0,
			"is_active" : false,
			"name" : "",
			"thumbnail?" : [ "Str", null ],
		} ],
		{
            //Only the names reserved by system can start with a capital letter.
			"ID" : 0, //ID is needed to identify the items and calculate the diff
			"created" : "2/24/2021",
			"id" : 1, //So this "id" is not necessary...
			"is_active" : false,
			"name" : "Holt Gianuzzi",
			"thumbnail" : [ "Str", null ],
		},
		{
			"ID" : 1,
			"created" : "5/28/2020",
			"id" : 2,
			"is_active" : true,
			"name" : "Sansone Gerard",
			"thumbnail" : [ "Str", null ],
		},
		{
			"ID" : 2,
			"created" : "3/14/2020",
			"id" : 3,
			"is_active" : false,
			"name" : "Max Mangan",
			"thumbnail" : [ "Str", null ],
		},
		{
			"ID" : 3,
			"created" : "11/24/2020",
			"id" : 4,
			"is_active" : false,
			"name" : "Brice Bartosch",
			"thumbnail" : [ "Str", null ],
		},
		{
			"ID" : 4,
			"created" : "4/26/2020",
			"id" : 5,
			"is_active" : true,
			"name" : "Lauretta Tyt",
			"thumbnail" : [ "Str", null ],
		}
	],
}
```
The dochy data to make this output is 157 bytes, while the original JSON5 file is 626 bytes and
215 bytes using flate2::ZlibEncoder;

Actually, the Dochy language only needs values changed from the default value. 
We can omit unchanged values, and it makes the file size 155 bytes.
```json5
{
  "Old" : [],
  "users" : [
    "MList",
    [ "NextID", 5 ],
    [ {
        "created" : "",
        "id" : 0,
        "is_active" : false,
        "name" : "",
        "thumbnail?" : [ "Str", null ],
      } ],
    {
      "ID" : 0,
      "created" : "2/24/2021",
      "id" : 1,
      "name" : "Holt Gianuzzi",
      //thumbnail and is_active are omitted because the values are unchanged from the default object. 
    },
    {
      "ID" : 1,
      "created" : "5/28/2020",
      "id" : 2,
      "is_active" : true,
      "name" : "Sansone Gerard",
    },
    //...
  ],
}
```
The difference is 2 bytes, which is very small because Dochy records some data bitwise. 
We omitted 8 items, but the values are boolean and null, and it occupied about 2 bytes. 

The compressed JSON5 has the dynamic type data too, 
and it's corresponding to the initial JSON5 in Dochy. The initial JSON5 is 139 bytes. 
155 + 139 = 273 > 215, so you may think this is not good enough.

(Type data is basically fixed, and it doesn't need re-download or such, though.)

Writing everything is not where the diff system really shines. Let's add one item to the list.
```json5
{
  "Old" : [],
  "users" : [
    "MList",
    [ "NextID", 6 ],
    [ {
        "created" : "",
        "id" : 0,
        "is_active" : false,
        "name" : "",
        "thumbnail?" : [ "Str", null ],
      } ],
    {
      "ID" : 0,
      "created" : "2/24/2021",
      "id" : 1,
      "name" : "Holt Gianuzzi",
    },
    //...
    {
      "ID" : 5,
      "created" : "6/11/2020",
      "id" : 6,
      "is_active" : true,
      "name" : "Pete Blincow",
    }
  ],
}
```
The Dochy file of this diff is 37 bytes, but in JSON, you should need to write the entire list to record the change.

There's a module dedicated to accumulate small changes efficiently. [fs::history](./history.md)

