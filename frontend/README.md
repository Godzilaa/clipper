# VideoClip Frontend

This is the landing page for the VideoClip CLI tool. It provides installation instructions, features overview, and usage examples.

## 🚀 How to Use

### Option 1: Open Locally
Simply open `index.html` in your web browser:
```bash
cd frontend
open index.html  # macOS
xdg-open index.html  # Linux
start index.html  # Windows
```

### Option 2: Serve with Python
```bash
cd frontend
python3 -m http.server 8000
# Then visit http://localhost:8000
```

### Option 3: Serve with Node.js
```bash
cd frontend
npx serve
```

### Option 4: Deploy to GitHub Pages

1. Push the frontend folder to your repository
2. Go to your repository Settings > Pages
3. Select the branch and `/frontend` folder
4. Your site will be live at `https://godzilaa.github.io/clipper/`

### Option 5: Deploy to Netlify/Vercel

Simply drag and drop the `frontend` folder to [Netlify](https://app.netlify.com/drop) or connect your GitHub repo to [Vercel](https://vercel.com).

## 📝 Customization

- **GitHub Link**: Already configured to https://github.com/Godzilaa/clipper
- **Add Analytics**: Add Google Analytics or Plausible tracking code before `</head>`
- **Change Colors**: Modify CSS variables in `styles.css` under `:root`
- **Add Features**: Extend the feature grid with more cards in `index.html`

## 🎨 Features

- ✅ Responsive design (mobile-friendly)
- ✅ Copy-to-clipboard functionality
- ✅ Smooth animations
- ✅ Tab-based platform instructions
- ✅ Modern gradient design
- ✅ Zero dependencies (vanilla JS)

## 📦 Files

- `index.html` - Main landing page
- `styles.css` - All styling and responsive design
- `script.js` - Interactive functionality
- `README.md` - This file
