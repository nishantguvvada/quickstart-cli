# 🛠️ FS-Quickstart CLI

**FS-Quickstart CLI** is a simple command-line tool that allows you to quickly scaffold a backend (in **Python** or **Node.js**) or a frontend (in **React** or **Next.js**) project with minimal setup.

---

## 🚀 Features

- Quickly initialize a **backend** project using Python (**FastAPI**) or Node.js.
- Quickly initialize a **frontend** project using React or Next.js.
- Simple and consistent CLI interface.

---

## 📥Install

```bash
cargo install fs-quickstart
```

---

## 📦 Usage

### 🧱 Command Structure

```bash
fs-quickstart <PROJECT_NAME> <PROJECT_TYPE> [options]
```

| Argument       | Description             |
| -------------- | ----------------------- |
| `PROJECT_NAME` | Name of your project    |
| `PROJECT_TYPE` | `frontend` or `backend` |

---

### ⚙️ Options

| Option                 | Description                                             | Values             |
| ---------------------- | ------------------------------------------------------- | ------------------ |
| `--backend-framework`  | Specify backend framework                               | `python` or `node` |
| `--fastapi`            | (Optional) Use FastAPI when backend framework is Python | _(flag)_           |
| `--frontend-framework` | Specify frontend framework                              | `react` or `next`  |

---

## 🧪 Examples

### 🔧 Create a Python backend with FastAPI

```bash
fs-quickstart myapp backend --backend-framework python --fastapi
```

### ⚙️ Create a Node.js backend

```bash
fs-quickstart myapp backend --backend-framework node
```

### 🎨 Create a React frontend

```bash
fs-quickstart myapp frontend --frontend-framework react
```

### 🌐 Create a Next.js frontend

```bash
fs-quickstart myapp frontend --frontend-framework next
```

---

✅ **Tip:** Use the `--fastapi` flag only if your backend framework is Python.

---
