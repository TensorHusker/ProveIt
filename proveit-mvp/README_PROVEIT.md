# ProveIt MVP - React Flow Proof Canvas

A visual proof-building interface using React Flow, built with TypeScript and Tailwind CSS.

## ✅ Phase 1 Complete!

All acceptance criteria have been implemented:

1. ✅ Three node types render with correct colors (Assumption/Blue, Rule/Green, Goal/Orange)
2. ✅ Can drag nodes around canvas
3. ✅ Can connect nodes with lines
4. ✅ Can add new nodes via toolbar
5. ✅ Connections show visual feedback (highlighted on hover)
6. ✅ Can double-click to edit node text
7. ✅ Canvas has zoom/pan controls
8. ✅ "Clear All" button works
9. ✅ Runs smoothly on desktop browser
10. ✅ Basic touch support implemented

## Getting Started

```bash
# The development server is already running!
# Open your browser to: http://localhost:5173/

# If you need to restart:
npm run dev
```

## Project Structure

```
proveit-mvp/
├── src/
│   ├── components/
│   │   ├── nodes/
│   │   │   ├── AssumptionNode.tsx    # Blue assumption nodes
│   │   │   ├── RuleNode.tsx          # Green rule nodes
│   │   │   └── GoalNode.tsx          # Orange goal nodes
│   │   └── Toolbar.tsx               # Left sidebar controls
│   ├── types/
│   │   └── nodes.ts                  # TypeScript definitions
│   ├── App.tsx                       # Main canvas component
│   └── index.css                     # Tailwind styles
├── tailwind.config.js
├── postcss.config.js
└── package.json
```

## Features Implemented

### Three Custom Node Types

1. **Assumption Node (Blue)** 📌
   - Represents logical assumptions
   - Has output handle on right side
   - Can connect to Rules or Goals

2. **Rule Node (Green)** ⚡
   - Represents inference rules
   - Configurable number of inputs/outputs
   - Can connect from Assumptions or other Rules
   - Can connect to Goals or other Rules

3. **Goal Node (Orange)** 🎯
   - Represents proof goals
   - Has validation indicator (○ when unvalidated)
   - Has input handle on left side

### Toolbar Actions

- **Add Assumption**: Creates new assumption node
- **Add Rule**: Creates new rule node
- **Add Goal**: Creates new goal node
- **Check Proof**: Placeholder for validation (Phase 2)
- **Clear All**: Removes all nodes (with confirmation)
- **Export JSON**: Downloads proof as JSON file

### Canvas Features

- **Drag & Drop**: All nodes are draggable
- **Pan & Zoom**: Use mouse wheel or controls
- **Connect Nodes**: Click and drag from handles
- **Edit Content**: Double-click any node to edit text
- **Valid Connections**: Only allows logically valid connections
- **Background Grid**: Visual grid for alignment
- **Smooth Connections**: Animated connection lines

### Initial Example

The canvas starts with 3 pre-connected nodes demonstrating:
- Assumption: "A ∧ B"
- Rule: "∧-Elimination-Left"
- Goal: "A"

## Node Editing

Double-click any node to edit:
- **Label**: The node's identifier
- **Proposition** (Assumption/Goal): The logical formula
- **Rule** (Rule nodes): The inference rule name

Press **Enter** or **click outside** to save changes.

## Connection Rules

Valid connection patterns:
- Assumption → Rule
- Assumption → Goal
- Rule → Rule
- Rule → Goal

Invalid connections are automatically prevented.

## Keyboard Shortcuts

- **Mouse Wheel**: Zoom in/out
- **Click + Drag**: Pan canvas
- **Double Click**: Edit node
- **Enter**: Save node edits

## Touch Support

Basic touch support for iPad/tablets:
- Tap to select
- Drag to move nodes
- Pinch to zoom (via React Flow)
- Tap to connect handles

## Next Steps (Phase 2)

The following features are planned but not yet implemented:

- [ ] Proof validation logic
- [ ] Rule library/dropdown
- [ ] Save/load functionality
- [ ] Undo/redo
- [ ] Node deletion (individual)
- [ ] Connection validation feedback
- [ ] Type checking
- [ ] Error messages
- [ ] Advanced styling/animations

## Tech Stack

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool & dev server
- **React Flow 11** - Node-based canvas
- **Tailwind CSS v3** - Styling (using stable version)

## Development

```bash
# Install dependencies
npm install

# Start dev server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Files Created

- Custom node components in `src/components/nodes/`
- Toolbar component in `src/components/`
- Type definitions in `src/types/`
- Tailwind configuration
- PostCSS configuration
- Updated App.tsx with React Flow canvas

## Known Limitations

- Node content editing doesn't persist to React Flow state (local state only)
- No validation logic yet (Phase 2)
- No save/load functionality (Phase 2)
- Mobile/iPad support is basic (Phase 2 will improve)

## Testing Checklist

✅ Can add nodes via toolbar
✅ Can drag nodes around
✅ Can connect nodes
✅ Can edit node text
✅ Can clear all nodes
✅ Can export to JSON
✅ Canvas zooms and pans smoothly
✅ Invalid connections are prevented
✅ Server starts without errors

---

**Phase 1 Status: COMPLETE** 🎉

The application is ready for your testing! Open http://localhost:5173/ in your browser.
