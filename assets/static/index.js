const searchInput = document.getElementById("search-input");
const shortButton = document.getElementById("shorten-button");
const resultArea = document.getElementById("result-area");

async function handler() {
  const query = searchInput.value;
  if (query) {
    const para = document.createElement("p");
    try {
      const dataJson = await putData(query);
      para.innerText = dataJson.short_url;
      resultArea.appendChild(para);
      searchInput.value = "";
    } catch (err) {
      console.error(err);
    }
  }
}

async function putData(query) {
  const request = await fetch("/api", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ url: query }),
  });
  if (!request.ok) {
    const err = await request.text();
    throw new Error(err);
  }
  return await request.json();
}

shortButton.addEventListener("click", handler);
searchInput.addEventListener("keypress", (event) => {
  if (event.key === "Enter") {
    handler();
  }
});
