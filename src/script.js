document
  .getElementById("downloadForm")
  .addEventListener("submit", async function (e) {
    e.preventDefault();

    const video = document.getElementById("video").value;
    const tipo = document.getElementById("tipo").value;
    const formato = document.getElementById("formato").value;
    const folder = document.getElementById("folder").value;

    const responseMessage = document.getElementById("responseMessage");
    responseMessage.textContent = "Processando...";
    responseMessage.classList.remove("success", "error");

    try {
      const response = await fetch(`${window.location.origin}/submit`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ video, tipo, formato, folder }),
      });

      if (!response.ok) {
        // Se o servidor retornar um erro (não for 2xx)
        throw new Error("Erro ao acessar o servidor.");
      }

      const data = await response.json();

      if (data.success) {
        responseMessage.textContent = "Download bem-sucedido!";
        responseMessage.classList.add("success");
      } else {
        responseMessage.textContent = "Erro ao baixar o vídeo.";
        responseMessage.classList.add("error");
      }
    } catch (error) {
      responseMessage.textContent = "Erro ao conectar ao servidor.";
      responseMessage.classList.add("error");
      console.error("Erro ao fazer a requisição:", error); // Exibe o erro no console
    }
  });
console.log(`${window.location.origin}/submit`);
