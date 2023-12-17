<div align="center">
<br/>

# Savoir

### Create AI assistants with ease.

<br/>
</div>

<div align="center">

[![PRs Welcome](https://img.shields.io/badge/PRs-Are%20welcome-brightgreen.svg?style=flat-square)](https://makeapullrequest.com) [![License](https://img.shields.io/github/license/kerwanp/savoir?label=License&style=flat-square)](LICENCE)

[🔨 Install](#🔨-install) • [🚀 Get started](#🚀-get-started) • [📁 Datasources](#📁-datasources) • [🎲 Integrations](#🎲-integrations) • [⚛ LLMs](#⚛-llms) • [📚 Documents stores](#📚-documents-stores)

[Contribute](#contributing) • [License](#license)

</div>

# 🔨 Install 

WIP

# 🚀 Get started 

```yaml
datasources:
  google:
    type: google
    service_account: ./service-account.json
    subject: john.doe@example.org

llms:
  openai:
    type: openai
    model: gpt-3.5-turbo-1106
    api_key: <your_api_key>

store:
  type: weaviate
  host: http://localhost:8080

agents:
  default:
    llm: openai
    prompt: "You are an helpful assistant that answer the collaborators questions using the following documents. If you do not find an answer in the documents, you simply answer that you do not have enough informations."

integrations:
  slack:
    type: slack
    agent: default
    channel: "#assistant"
    signing_secret: <signing_secret>
    port: 8081
```

```bash
$ savoir synchronize google # Start synchronizing the google datasource
$ savoir ask default "Who is in charge of designing the new landing page?" # Directly ask questions from the command-line
$ savoir serve slack # Start running the Slack integration
```
# 📁 Datasources

| Datasource       | Status         |
| ---------------- | -------------- |
| Google Drive     | 🔶 Alpha       |
| Notion           | ❌ Planned     |

# 🎲 Integrations

| Integration      | Status         |
| ---------------- | -------------- |
| Slack            | 🔶 Alpha       |
| Discord          | ❌ Planned     |

# ⚛ LLMs

| LLM              | Status         |
| ---------------- | -------------- |
| OpenAI           | 🔶 Alpha       |

# 📚 Documents stores

| Documents stores | Status         |
| ---------------- | -------------- |
| Weaviate         | 🔶 Alpha       |
| Elasticsearch    | ❌ Planned     |

# Contributing

I'd love for you to contribute to this project. You can request new features by creating an issue, or submit a pull request with your contribution.

# Licence

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

```
http://www.apache.org/licenses/LICENSE-2.0
```

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
