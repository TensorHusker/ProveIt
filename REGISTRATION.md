# User Registration Form

An accessible, user-friendly registration form for the ProveIt application.

## Features

### Accessibility-First Design

This registration form follows WCAG 2.1 Level AA guidelines and includes:

- **Screen Reader Support**: Proper ARIA labels, live regions for error announcements, and semantic HTML
- **Keyboard Navigation**: Full keyboard accessibility with visible focus indicators
- **High Contrast Mode**: Automatic support for high contrast preferences
- **Reduced Motion**: Respects user's reduced motion preferences
- **Dark Mode**: Automatic dark mode support based on system preferences
- **Accessibility Preferences**: Built-in options for users to customize their experience

### Form Validation

- **Real-time Validation**: Errors are shown as users fill out the form
- **Clear Error Messages**: Descriptive error messages for each field
- **Required Fields**: Clearly marked with asterisks
- **Password Strength**: Requirements for secure passwords (minimum 8 characters, letters and numbers)

### User Experience

- **Responsive Design**: Works on mobile, tablet, and desktop devices
- **Auto-complete**: Proper autocomplete attributes for password managers
- **Help Text**: Clear instructions for each field
- **Success Feedback**: Clear confirmation when registration is successful

## Files

- `registration.html` - Main HTML structure
- `registration.css` - Styling with accessibility features
- `registration.js` - Client-side validation and form handling

## Usage

Simply open `registration.html` in a web browser to use the registration form.

### Form Fields

**Required Fields:**
- **Username**: 3-20 characters, letters and numbers only
- **Email Address**: Valid email format
- **Password**: Minimum 8 characters, must include letters and numbers
- **Confirm Password**: Must match the password
- **Terms Agreement**: Must accept terms and conditions

**Optional Fields:**
- **High Contrast Mode**: Enable for better visibility
- **Screen Reader Optimization**: Optimize interface for screen readers
- **Reduce Motion**: Minimize animations and transitions

## Technical Details

### Accessibility Features

1. **ARIA Attributes**:
   - `aria-required` for required fields
   - `aria-describedby` linking inputs to help text and errors
   - `aria-invalid` for validation state
   - `aria-live` for dynamic error announcements

2. **Semantic HTML**:
   - Proper `<label>` associations
   - `<fieldset>` and `<legend>` for grouped inputs
   - Heading hierarchy for screen reader navigation

3. **Focus Management**:
   - Visible focus indicators (3px outline)
   - Automatic focus on first field
   - Focus on first error when validation fails

4. **Visual Accessibility**:
   - Minimum 4.5:1 color contrast ratio
   - Clear visual hierarchy
   - Large touch targets (minimum 44x44px)

### Browser Support

The form works in all modern browsers including:
- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Mobile browsers

### Integration

To integrate this form into an existing application:

1. Update the form submission handler in `registration.js` to send data to your backend
2. Replace the placeholder links for Terms of Service and Privacy Policy
3. Customize the color scheme in `registration.css` to match your brand
4. Add server-side validation matching the client-side rules

## Example Integration

```javascript
// In registration.js, replace the handleSuccessfulRegistration function:

function handleSuccessfulRegistration() {
    const formData = {
        username: usernameInput.value,
        email: emailInput.value,
        password: passwordInput.value,
        accessibility: {
            highContrast: document.getElementById('highContrast').checked,
            screenReader: document.getElementById('screenReader').checked,
            reduceMotion: document.getElementById('reduceMotion').checked
        }
    };

    fetch('/api/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData)
    })
    .then(response => response.json())
    .then(data => {
        form.classList.add('hidden');
        successMessage.classList.remove('hidden');
        successMessage.focus();
    })
    .catch(error => {
        console.error('Registration error:', error);
        alert('Registration failed. Please try again.');
    });
}
```

## Testing

### Manual Testing Checklist

- [ ] All required fields show errors when empty
- [ ] Username validation works (length, characters)
- [ ] Email validation accepts valid emails
- [ ] Password validation enforces requirements
- [ ] Confirm password matches password field
- [ ] Terms checkbox must be checked
- [ ] Form submits successfully with valid data
- [ ] Form reset clears all fields and errors
- [ ] Tab navigation works through all fields
- [ ] Screen reader announces errors
- [ ] Focus indicators are visible
- [ ] Works on mobile devices
- [ ] Dark mode displays correctly
- [ ] High contrast mode is accessible

## License

MIT License - Same as the ProveIt project
