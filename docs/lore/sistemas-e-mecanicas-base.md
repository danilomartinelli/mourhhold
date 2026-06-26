# Sistemas e Mecânicas Base

Este documento consolida a direção sistêmica de Mournhold antes da implementação. O jogo combina RPG 2D pixelado, regiões semi-livres, combate por turnos com timing ativo, loot/build inspirado em Diablo IV e exploração de lore por Ecos, ruínas e Aspectos.

## Identidade sistêmica

Frase-guia:

> Um RPG 2D open world em regiões semi-livres focado em exploração, progressão de personagem, descoberta de lore e reconstrução de uma história perdida através de Ecos, ruínas ancestrais e escolhas que moldam a relação de Corin Purake com a Primeira Noite.

O loop principal desejado é:

```text
explorar
↓
encontrar inimigos/segredos/dungeons
↓
batalhar
↓
loot
↓
ajustar build
↓
desbloquear rotas/lore
↓
buscar desafios melhores
```

## Party e companheiros

O jogo não possui party fixa.

Corin é sempre o eixo narrativo e mecânico. Outros personagens entram como:

- companheiros temporários;
- mentores;
- aliados de facção;
- interesses políticos;
- personagens de arco específico.

Exemplo de estrutura:

```text
Prólogo: Corin + Elara/Bran
Ato 1: Corin + aliado temporário/mercenário
Ato 2: Corin + Joruun
Ato 3: Corin + Umbrakai
Ato 4: Corin + Elara novamente
Final: composição depende de escolhas e alianças
```

Aliados temporários podem contribuir em combate, gerar Foco e participar de combos, mas não definem uma party permanente.

## Mundo e exploração

O mundo usa regiões semi-livres, não open world totalmente plano.

Exemplos de regiões:

```text
Continente de Aethra
├── Ravenhollow
├── Marca de Corvhal
├── Valecrown
├── Planícies Arkanas
├── Terras Congeladas Joruun
├── Ruínas Umbrakai
├── Costa Sombria
├── Floresta de Nhar
└── Desfiladeiro do Véu
```

O jogo usa gating suave. Áreas podem ser acessíveis cedo, mas perigosas por:

- inimigos de nível alto;
- clima;
- Suspeita Imperial;
- falta de skill de exploração;
- reputação insuficiente;
- puzzles ainda ilegíveis;
- rotas bloqueadas por fortalezas.

## Combate

O combate é por turnos clássicos com timing ativo opcional.

Base:

- menu de ações;
- ordem de turnos clara;
- ataque;
- defesa;
- skills;
- itens;
- fuga quando permitido;
- timing em ataques e defesas para bônus.

O timing ativo é importante, mas não obrigatório. Ele pode melhorar:

- dano;
- mitigação;
- geração de Véu;
- geração de Foco;
- chance de status;
- efeitos extras de skills;
- contra-ataques específicos.

A vitória deve depender principalmente de:

- build;
- gerenciamento de recursos;
- leitura de inimigos;
- equipamentos;
- itens;
- preparação.

## Recursos de combate

### Vigor

Recurso usado para:

- skills físicas;
- esquiva;
- defesa ativa;
- técnicas de arma;
- ações de sobrevivência.

### Véu

Recurso sobrenatural ligado às manifestações Purake de Corin.

Progressão:

```text
Prólogo
↓
Véu bloqueado

Primeiro Eco
↓
Véu aparece como recurso incompleto/instável

Ato 1
↓
Manifestações do Véu limitadas

Arcos posteriores
↓
Árvores formais do Véu
```

No Ato 1, habilidades do Véu são poucas e parcialmente controláveis:

- Sombra Instintiva — defesa emergencial/redução de dano quando Corin está em perigo.
- Pulso do Véu — ataque fraco em área, custo alto.
- Percepção de Eco — exploração; revela rastros, segredos e pontos de memória.

### Foco

Recurso compartilhado do combate.

Gerado por:

- timing perfeito;
- defesa bem-sucedida;
- leitura de inimigos;
- ações coordenadas com aliados temporários.

Usado em:

- ações táticas;
- combos;
- respostas defensivas;
- habilidades especiais de aliados;
- viradas de turno.

Quando Corin está sozinho, Foco continua funcionando. Quando há aliados temporários, todos podem contribuir para o mesmo recurso.

## Encontros e arenas

Não há batalhas aleatórias clássicas.

Inimigos aparecem visíveis no mapa. O jogador pode:

- engajar;
- evitar;
- preparar emboscada;
- perseguir elites;
- fugir de patrulhas;
- usar skills de exploração para obter vantagem.

Ao tocar um inimigo, o jogo transiciona para uma arena separada estilizada, com fundo baseado no bioma/local atual.

A posição no mapa influencia o começo da batalha:

- Corin pega inimigo por trás: emboscada do jogador.
- Inimigo pega Corin por trás/lateral: emboscada inimiga.
- Contato frontal: batalha normal.
- Skill de exploração: vantagem especial.

## Status de combate

Estados devem misturar efeitos físicos, táticos, arcanos e ligados ao Véu.

Exemplos:

- Sangramento;
- Quebrado;
- Atordoado;
- Lentidão;
- Exposto;
- Marcado;
- Congelado;
- Ecoado;
- Velado;
- Corrompido pelo Véu.

## Progressão de personagem

Corin não possui classe escolhível. Ele é sempre Corin Purake.

Dois jogadores podem terminar o jogo com builds completamente diferentes por meio de:

- skills;
- talentos;
- equipamentos;
- Aspectos;
- Relíquias;
- reputações;
- progressão do Véu;
- Convergência no pós-campanha.

### Níveis

Nível máximo da campanha: aproximadamente **50**.

Ao subir de nível, Corin recebe:

- atributos base automaticamente;
- 1 Ponto de Talento.

### Atributos principais

- Força — dano físico.
- Agilidade — velocidade, evasão e timing.
- Vontade — poder do Véu.
- Percepção — chance/qualidade de efeitos de Eco.
- Afinidade — ligação com a Primeira Noite.

### Talentos no Ato 1

No Ato 1, Pontos de Talento alimentam a árvore geral de **Sobrevivente de Corvhal**.

Focos:

- Vigor;
- contra-ataque com timing;
- uso melhor de itens;
- esquiva;
- resistência ao frio;
- técnicas simples de arma;
- recuperação ao defender.

Árvores místicas ficam para depois.

### Árvores futuras do Véu

As árvores formais não aparecem no Ato 1. Elas devem desbloquear por revelação narrativa.

Árvores previstas:

#### Ouvinte

Percepção, controle, leitura de Ecos e suporte.

Exemplos:

- Sentir Ecos;
- Marca Oculta;
- Visão do Véu;
- Eco Reverso.

#### Caminhante do Véu

Mobilidade, evasão, posicionamento e estilo rogue.

Exemplos:

- Passo Sombrio;
- Teleporte curto;
- Clone Sombrio;
- Distorção.

#### Portador da Primeira Noite

Poder bruto, habilidades da Primeira Noite e alto risco.

Exemplos:

- Eclipse;
- Manto da Noite;
- Corrupção do Véu;
- Convocar Ecos.

## Convergência

Após o nível máximo da campanha, Corin progride por **Convergência**, um sistema semelhante ao Paragon.

Função:

- estender progressão sem inflar níveis indefinidamente;
- criar builds avançadas;
- conectar pós-game ao Véu e à Primeira Noite;
- sustentar dungeons endgame e chefes lendários.

Exemplo conceitual:

```text
Primeira Noite
        |
Ouvinte --- Véu
        |
Portador
```

## Equipamentos

Slots:

```text
Arma
Capuz
Peitoral
Luvas
Botas
Amuleto
Anel
Anel
Relíquia
```

Raridades:

```text
Comum → Mágico → Raro → Lendário → Relíquia → Mítico
```

Função por raridade:

- Comum — combustível de desmontagem.
- Mágico — leveling inicial e desmontagem.
- Raro — base de progressão durante campanha.
- Lendário — contém Aspectos.
- Relíquia — item único, fixo e aprimorável.
- Mítico — raríssimo, ligado a lore e endgame.

## Loot

Loot é frequente, mas filtrado por contexto.

- Inimigos comuns: materiais, ouro, consumíveis, chance baixa de mágico/raro.
- Elites: boa chance de raro/lendário.
- Bosses: drop relevante garantido.
- Dungeons: baú final com loot direcionado.
- Fortalezas/Ecos Ancestrais: melhores fontes de Aspectos, Relíquias e Míticos.

## Inventário

Inventário praticamente ilimitado para equipamentos, materiais, objetos de missão e lore.

Sem peso e sem limite pequeno de slots para loot.

### Bolsa do Viajante

Categorias:

```text
Equipamentos
Relíquias
Materiais
Consumíveis
Objetos de Missão
```

### Limites

- Equipamentos: ilimitado.
- Materiais: ilimitado.
- Objetos de missão: ilimitado.
- Ecos/lore: coleção separada.
- Consumíveis: limite pequeno por tipo.

### Sistemas obrigatórios

- filtros;
- busca;
- favoritos;
- desmontagem em massa;
- Codex de Aspectos;
- Baú Global.

Itens favoritos não podem ser vendidos, desmontados ou destruídos.

## Consumíveis

Consumíveis têm limite pequeno por tipo, para gerar decisões táticas.

Categorias:

- Poções de Vida;
- Poções do Véu;
- Tônicos de Vigor;
- Elixires temporários;
- antídotos/curas de status;
- itens de exploração.

## Desmontagem e crafting

O sistema principal é desmontagem + melhoria, não crafting livre complexo.

Itens desmontados viram materiais por:

- tipo;
- raridade;
- origem;
- categoria de Aspecto.

Materiais servem para:

- melhorar equipamentos;
- aprimorar Relíquias;
- melhorar Codex de Aspectos;
- criar consumíveis;
- ativar receitas especiais.

Crafting completo fica limitado a:

- receitas especiais;
- Relíquias;
- itens Míticos;
- consumíveis avançados.

## Codex de Aspectos

Aspectos são modificadores lendários de build e fragmentos de história.

Ao encontrar um lendário, o Aspecto pode ser extraído para o Codex. A extração é permanente, mas há progressão de qualidade.

Exemplo:

```text
Primeiro lendário
↓
desbloqueia Aspecto no Codex

Versões melhores
↓
melhoram valores/intervalos do Aspecto
```

Cada Aspecto possui:

- efeito mecânico;
- fragmentos históricos progressivos;
- origem temática;
- coleção tipo Fragmento 1/5, 2/5 etc.

Exemplo:

```text
Aspecto do Primeiro Eco

Mecânica:
Passo Sombrio gera uma onda que causa dano em área.

Fragmento 1/5:
“Quando a Primeira Noite voltou sua atenção para o mundo, os sinos de Vael'Ruun tocaram sem mãos para movê-los.”
```

Fontes de Aspectos:

- dungeons;
- bosses;
- Ecos;
- facções;
- side quests;
- endgame.

Recompensas por completar conjuntos:

- conquistas;
- cosméticos;
- títulos;
- Ecos especiais;
- dungeons secretas.

## Categorias e slots de Aspectos

Aspectos não podem ser aplicados livremente em qualquer slot. O sistema usa categorias e slots permitidos.

Categorias:

- Ofensivo;
- Defensivo;
- Mobilidade;
- Véu;
- Eco;
- Utilitário;
- Lendário/Ancestral;
- Purake;
- Primeira Noite.

Slots:

```text
ARMA
Ofensivo / Véu

CAPUZ
Eco / Utilitário

PEITORAL
Defensivo / Véu

LUVAS
Ofensivo / Mobilidade

BOTAS
Mobilidade / Utilitário

ANEL
Eco / Véu / Utilitário

AMULETO
Qualquer Aspecto
+50% poder do Aspecto

RELÍQUIA
Aspectos Ancestrais
Aspectos Purake
Aspectos da Primeira Noite
```

Aspectos de Eco só entram em Amuleto ou Relíquia. Aspectos Purake são raros e poderosos. Relíquias Purake de endgame podem conter dois Aspectos.

Alguns Aspectos alteram não só números, mas a relação mecânica/narrativa de Corin com o Véu, a Primeira Noite e os Ecos.

## Relíquias

Relíquias são únicas, com identidade fixa, mas aprimoráveis.

Elas:

- não são loot descartável;
- não têm rolls aleatórios comuns;
- carregam lore própria;
- podem evoluir/despertar com materiais, reputação, Ecos, dungeons ou marcos narrativos;
- alteram builds profundamente.

Exemplos:

- Medalhão de Perrin;
- Fragmento Purake;
- Gelo Joruun;
- Relíquias Purake de endgame com dois Aspectos.

## Puzzles

Puzzles possuem três camadas:

- Caminho principal: simples/médios, sem travar ritmo.
- Caminhos opcionais: mais difíceis, recompensando Aspectos, Relíquias, Ecos e lore.
- Endgame: complexos, multi-etapa e possivelmente ligados a regiões/dungeons diferentes.

Famílias principais:

### Espelhos/Sombras

Alinhar luz, sombra e símbolos do Véu.

### Ecos Temporais

Observar uma memória e replicar ações no presente.

### Runas Purake

Sequência, padrão ou tradução parcial.

### Ambiente/Exploração

Empurrar objetos, abrir rotas, ativar mecanismos.

### Reputação/Conhecimento

Soluções alternativas se o jogador tiver facção, lore ou Codex suficiente.

## Dungeons

Dungeons são híbridas por tipo.

- Campanha principal: artesanais.
- Dungeons opcionais importantes: artesanais.
- Ecos Ancestrais/endgame: semi-procedurais com modificadores.
- Fortalezas: artesanais, com estados antes/depois.

Cada dungeon deve entregar:

- mecânica nova;
- lore nova;
- equipamento novo;
- boss ou desafio final.

## Fortalezas

Fortalezas são áreas regionais ocupadas por Império, criaturas do Véu, bandidos ou forças antigas.

Ao limpar uma fortaleza:

- muda estado do mapa;
- libera cidade/refúgio;
- libera mercador;
- desbloqueia dungeon ou rota;
- altera reputação regional;
- pode reduzir Suspeita Imperial local ou aumentar retaliação.

Exemplo:

```text
Fortaleza de Corvhal
Dominada por criaturas do Véu.

Ao limpar:
- libera refúgio;
- abre rota de viagem;
- desbloqueia dungeon opcional;
- aumenta confiança dos Refugiados de Corvhal.
```

## Ecos exploráveis

Ecos funcionam como memórias exploráveis.

Corin encontra um local, objeto, relíquia ou fenômeno do Véu e ativa um Eco. Ecos revelam:

- lore;
- segredos;
- tesouros;
- soluções de puzzle;
- fragmentos do Codex;
- caminhos ocultos.

Tipos:

### Eco Observado

Cena ou memória curta.

### Eco Interativo

Jogador repete ações do passado para alterar o presente ou revelar algo.

### Eco Fragmentado

Puzzle de reconstrução de memória.

### Eco Ancestral

Dungeon/endgame com modificadores.

## Reputação

Reputação é por facção e/ou região, não karma global.

Ela deve ser apresentada por níveis nomeados e feedback diegético, sem números crus.

Ações que alteram reputação:

- salvar refugiados;
- entregar suprimentos;
- poupar ou derrotar patrulhas;
- completar contratos locais;
- resolver fortalezas;
- revelar ou esconder informações;
- usar poderes do Véu em público;
- ajudar ou trair facções.

No Ato 1, reputações visíveis:

- Refugiados de Corvhal;
- Valecrown;
- Suspeita Imperial.

## Suspeita Imperial

Suspeita Imperial não é reputação moral. Mede o quanto Corin é visível ou procurado pelo Império Arkano.

Exemplo de níveis:

```text
Ignorado → Suspeito → Procurado → Caçado
```

Sobe quando Corin:

- usa poderes estranhos em público;
- derrota patrulhas;
- ajuda refugiados procurados;
- invade instalações;
- deixa testemunhas imperiais;
- é associado a símbolos Purake.

Pode baixar localmente por:

- disfarces;
- subornos;
- rotas discretas;
- evitar testemunhas;
- reduzir presença imperial em uma fortaleza.

Marcos narrativos importantes nunca são apagados.

## Skills de exploração

Skills não devem ser só de combate. Algumas abrem rotas e segredos.

Exemplos:

- Percepção de Eco — revela memórias/rastros.
- Passo Sombrio — atravessa pequenas fendas/atalhos.
- Sombra Instintiva — evita armadilhas uma vez.
- Leitura de Runas — decifra inscrições.
- Véu Silente — reduz detecção por patrulhas.

## Side quests

Side quests devem cair em três categorias:

### Humanas

Refugiados, vilas, perdas, escolhas morais e consequências sociais.

### Sistêmicas

Fortalezas, contratos, elites, materiais, reputação e rotas.

### Lore

Ecos, ruínas, Aspectos, registros, facções e fragmentos históricos.

## Morte e derrota

Derrota não deve apagar progresso de loot/lore.

Ao morrer:

- retorna ao último Santuário/Refúgio;
- perde parte de ouro ou recursos temporários;
- mantém itens importantes, Aspectos, lore e progresso narrativo;
- em dungeons, pode perder progresso interno se não ativou marco/checkpoint.

## Save e checkpoints

Usar autosave + Santuários/Refúgios manuais.

Autosave em:

- transições;
- loot importante;
- fim de batalha;
- marcos narrativos;
- entrada/saída de dungeon.

Santuários/Refúgios permitem:

- salvar manualmente;
- trocar build;
- acessar Baú Global;
- desmontar itens;
- viajar rápido se desbloqueado.

## Endgame

O pós-jogo deve ser robusto.

Conteúdos:

- Ecos Ancestrais;
- dungeons perdidas;
- chefes lendários;
- reputações máximas;
- equipamentos míticos;
- Aspectos raros;
- Convergência avançada.

### Ecos Ancestrais

Versão endgame das dungeons modificadas.

Exemplo:

```text
Eco de Aruun +10

Modificadores:
- inimigos com 300% HP;
- Sombras adicionais;
- Véu instável;
- limite de consumíveis reduzido.

Recompensas:
- Relíquias únicas;
- Aspectos lendários;
- fragmentos históricos raros.
```

### Chefes lendários / mundiais

Exemplos:

- Guardião do Primeiro Eco;
- O Último Ouvinte;
- Vigia de Nul'Vareth;
- Aruun, o Vigia Congelado.

## New Game+

New Game+ recomendado.

Deve incluir:

- novos diálogos;
- novos Ecos;
- novas áreas;
- lore adicional;
- variações de encontros;
- recompensas cosméticas e míticas.

O objetivo é permitir que jogadores que conhecem a verdade vejam o começo do jogo com novos olhos, especialmente Ravenhollow, Kael Voss, Vael'Ruun e Perrin.
