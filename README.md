Este código fornece uma implementação em Rust para enumeração de subdomínios usando as APIs SecurityTrails e Shodan.

Para usar este código, você precisará ter uma conta e uma chave de API válida para acessar as APIs SecurityTrails e Shodan.

Requisitos
Rust
reqwest (biblioteca para fazer requisições HTTP)
serde (biblioteca para deserializar a resposta JSON das APIs)
serde_json (biblioteca para deserializar a resposta JSON das APIs)

Como usar

Clone este repositório e abra o arquivo "main.rs"

Substitua "SUA_CHAVE_DA_API" na linha 10 com sua própria chave de API válida para acessar as APIs.

Substitua "example.com" na linha 11 com o domínio para o qual você deseja obter os subdomínios.

Compile e execute o código com o comando cargo run
O código fará chamadas para as APIs SecurityTrails e Shodan para obter os subdomínios do domínio especificado. A resposta das APIs será deserializada em structs específicas e os subdomínios serão imprimidos na tela.

Explicação do código
As linhas 3-6 definem a struct SecurityTrailsResponse, que será usada para armazenar os dados de resposta da API SecurityTrails.

As linhas 7-9 definem a struct ShodanResponse, que será usada para armazenar os dados de resposta da API Shodan.

As linhas 10-13 definem a struct ShodanData, que será usada para armazenar os dados de cada item dentro da resposta da API Shodan.

A partir da linha 14, é feita uma chamada para a API SecurityTrails e a resposta é deserializada para a struct SecurityTrailsResponse. Os subdomínios são então impressos na tela.
A partir da linha 21, é feita uma chamada para a API Shodan e a resposta é deserializada para a struct ShodanResponse. Os subdomínios são então impressos na tela.

Observação: Este código é apenas um exemplo e pode ser modificado ou estendido para atender às suas necessidades específicas.
