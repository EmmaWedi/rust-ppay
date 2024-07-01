var db = connect("mongodb://admin:password@localhost:27017/admin");

db = db.getSiblingDB('PeoplesPay');

db.createUser(
    {
        user: "wedi",
        pwd: "wedime",
        roles: [ { role: "readWrite", db: "PeoplesPay"} ],
        passwordDigestor: "server",
    }
)