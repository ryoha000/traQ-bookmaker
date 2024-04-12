fetch("http://localhost:4351/message", {
  method: "POST",
  body: JSON.stringify({ message: "Hello, World!" }),
}).then((response) => console.log(response.status));
