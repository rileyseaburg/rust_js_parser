interface User {
    id: number;
    name: string;
    email: string;
}

function fetchUsers(): Promise<User[]> {
    return fetch('/api/users')
        .then(response => response.json())
        .then(data => data as User[]);
}

async function displayUsers() {
    try {
        const users = await fetchUsers();
        users.forEach(user => {
            console.log(`${user.name} - ${user.email}`);
        });
    } catch (error) {
        console.error('Failed to fetch users:', error);
    }
}

displayUsers();