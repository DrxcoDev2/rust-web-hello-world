document.getElementById("userForm").addEventListener("submit", async function(e) {
    e.preventDefault();

    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;
    const email = document.getElementById("email").value;

    const response = await fetch("/users", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ username, password, email })
    });

    const result = document.getElementById("result");

    if (response.ok) {
        result.textContent = "✅ Usuario creado correctamente";
        this.reset();
    } else {
        result.textContent = "❌ Error al crear el usuario";
    }
});

// Cargar la lista de usuarios al cargar la página
document.addEventListener("DOMContentLoaded", async function() {
    const response = await fetch("/users");
    const users = await response.json();
    const usersListContent = document.getElementById("usersListContent");
    usersListContent.innerHTML = "";
    users.forEach(user => {
        const li = document.createElement("li");
        li.textContent = user.username;
        usersListContent.appendChild(li);
    });
});