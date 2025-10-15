const handler = () => {
    const users = [{
            id: 1,
            name: "Ahmed",
            is_broke: true
        },
        {
            id: 2,
            name: "Galal",
            is_broke: true
        },
        {
            id: 3,
            name: "Eyad",
            is_broke: true
        },
        {
            id: 4,
            name: "bronny",
            is_broke: true
        },
        {
            id: 5,
            name: "Youssef",
            is_broke: true
        },
        {
            id: 6,
            name: "Fahd",
            is_broke: false
        },
    ];

    return JSON.stringify({
        status: 200,
        body: {
            data: {
                users: users,
                users_count: users.length
            }
        }
    });
}