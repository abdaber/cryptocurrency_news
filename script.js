async function searchNews() {
  const query = document.getElementById("search").value.trim();
  const container = document.getElementById("results");
  container.innerHTML = "<p>Loading news...</p>";

  try {
    console.log("Sending query:", query); // Debug log
    const apiUrl = `/api/news?q=${encodeURIComponent(query)}`;
    console.log("API URL:", apiUrl); // Debug log
    
    const res = await fetch(apiUrl);
    console.log("Response status:", res.status); // Debug log

    if (!res.ok) {
      throw new Error(`API request failed: ${res.status} ${res.statusText}`);
    }

    const data = await res.json();
    console.log("Received data:", data); // Debug log

    container.innerHTML = "";

    if (data.articles && data.articles.length > 0) {
      data.articles.forEach(article => {
        const el = document.createElement("div");
        el.className = "article";
        el.innerHTML = `
          <h3>${article.title}</h3>
          ${article.urlToImage ? `<img src="${article.urlToImage}" alt="${article.title}" style="max-width: 200px;">` : ''}
          <p>${article.description || "No description available"}</p>
          <p><small>${new Date(article.publishedAt).toLocaleDateString()} - ${article.source?.name || 'Unknown source'}</small></p>
          <a href="${article.url}" target="_blank">Read more</a>
        `;
        container.appendChild(el);
      });
    } else {
      container.innerHTML = "<p>No cryptocurrency news found. Try a different search term.</p>";
    }
  } catch (error) {
    console.error("Error fetching news:", error);
    container.innerHTML = `<p>Error loading news: ${error.message}</p>`;
  }
}

// Call on page load
document.addEventListener('DOMContentLoaded', searchNews);