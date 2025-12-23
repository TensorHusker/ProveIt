// Form validation and handling
(function() {
    'use strict';

    // Get form and elements
    const form = document.getElementById('registrationForm');
    const usernameInput = document.getElementById('username');
    const emailInput = document.getElementById('email');
    const passwordInput = document.getElementById('password');
    const confirmPasswordInput = document.getElementById('confirmPassword');
    const termsCheckbox = document.getElementById('terms');
    const successMessage = document.getElementById('successMessage');

    // Error message elements
    const usernameError = document.getElementById('username-error');
    const emailError = document.getElementById('email-error');
    const passwordError = document.getElementById('password-error');
    const confirmPasswordError = document.getElementById('confirmPassword-error');
    const termsError = document.getElementById('terms-error');

    // Validation functions
    function validateUsername(username) {
        if (!username || username.trim().length === 0) {
            return 'Username is required';
        }
        if (username.length < 3) {
            return 'Username must be at least 3 characters long';
        }
        if (username.length > 20) {
            return 'Username must not exceed 20 characters';
        }
        if (!/^[a-zA-Z0-9]+$/.test(username)) {
            return 'Username can only contain letters and numbers';
        }
        return '';
    }

    function validateEmail(email) {
        if (!email || email.trim().length === 0) {
            return 'Email address is required';
        }
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!emailRegex.test(email)) {
            return 'Please enter a valid email address';
        }
        return '';
    }

    function validatePassword(password) {
        if (!password || password.length === 0) {
            return 'Password is required';
        }
        if (password.length < 8) {
            return 'Password must be at least 8 characters long';
        }
        if (!/[a-zA-Z]/.test(password)) {
            return 'Password must contain at least one letter';
        }
        if (!/[0-9]/.test(password)) {
            return 'Password must contain at least one number';
        }
        return '';
    }

    function validateConfirmPassword(password, confirmPassword) {
        if (!confirmPassword || confirmPassword.length === 0) {
            return 'Please confirm your password';
        }
        if (password !== confirmPassword) {
            return 'Passwords do not match';
        }
        return '';
    }

    function validateTerms(checked) {
        if (!checked) {
            return 'You must agree to the terms and conditions';
        }
        return '';
    }

    // Display error message
    function showError(element, message) {
        if (element) {
            element.textContent = message;
            if (message) {
                element.setAttribute('aria-hidden', 'false');
            } else {
                element.setAttribute('aria-hidden', 'true');
            }
        }
    }

    // Clear error message
    function clearError(element) {
        showError(element, '');
    }

    // Add real-time validation listeners
    usernameInput.addEventListener('blur', function() {
        const error = validateUsername(this.value);
        showError(usernameError, error);
        if (error) {
            this.setAttribute('aria-invalid', 'true');
        } else {
            this.setAttribute('aria-invalid', 'false');
        }
    });

    usernameInput.addEventListener('input', function() {
        if (usernameError.textContent) {
            const error = validateUsername(this.value);
            showError(usernameError, error);
            if (error) {
                this.setAttribute('aria-invalid', 'true');
            } else {
                this.setAttribute('aria-invalid', 'false');
            }
        }
    });

    emailInput.addEventListener('blur', function() {
        const error = validateEmail(this.value);
        showError(emailError, error);
        if (error) {
            this.setAttribute('aria-invalid', 'true');
        } else {
            this.setAttribute('aria-invalid', 'false');
        }
    });

    emailInput.addEventListener('input', function() {
        if (emailError.textContent) {
            const error = validateEmail(this.value);
            showError(emailError, error);
            if (error) {
                this.setAttribute('aria-invalid', 'true');
            } else {
                this.setAttribute('aria-invalid', 'false');
            }
        }
    });

    passwordInput.addEventListener('blur', function() {
        const error = validatePassword(this.value);
        showError(passwordError, error);
        if (error) {
            this.setAttribute('aria-invalid', 'true');
        } else {
            this.setAttribute('aria-invalid', 'false');
        }
        // Also revalidate confirm password if it has a value
        if (confirmPasswordInput.value) {
            const confirmError = validateConfirmPassword(this.value, confirmPasswordInput.value);
            showError(confirmPasswordError, confirmError);
            if (confirmError) {
                confirmPasswordInput.setAttribute('aria-invalid', 'true');
            } else {
                confirmPasswordInput.setAttribute('aria-invalid', 'false');
            }
        }
    });

    passwordInput.addEventListener('input', function() {
        if (passwordError.textContent) {
            const error = validatePassword(this.value);
            showError(passwordError, error);
            if (error) {
                this.setAttribute('aria-invalid', 'true');
            } else {
                this.setAttribute('aria-invalid', 'false');
            }
        }
        // Also revalidate confirm password if it has a value
        if (confirmPasswordInput.value && confirmPasswordError.textContent) {
            const confirmError = validateConfirmPassword(this.value, confirmPasswordInput.value);
            showError(confirmPasswordError, confirmError);
            if (confirmError) {
                confirmPasswordInput.setAttribute('aria-invalid', 'true');
            } else {
                confirmPasswordInput.setAttribute('aria-invalid', 'false');
            }
        }
    });

    confirmPasswordInput.addEventListener('blur', function() {
        const error = validateConfirmPassword(passwordInput.value, this.value);
        showError(confirmPasswordError, error);
        if (error) {
            this.setAttribute('aria-invalid', 'true');
        } else {
            this.setAttribute('aria-invalid', 'false');
        }
    });

    confirmPasswordInput.addEventListener('input', function() {
        if (confirmPasswordError.textContent) {
            const error = validateConfirmPassword(passwordInput.value, this.value);
            showError(confirmPasswordError, error);
            if (error) {
                this.setAttribute('aria-invalid', 'true');
            } else {
                this.setAttribute('aria-invalid', 'false');
            }
        }
    });

    termsCheckbox.addEventListener('change', function() {
        const error = validateTerms(this.checked);
        showError(termsError, error);
        if (error) {
            this.setAttribute('aria-invalid', 'true');
        } else {
            this.setAttribute('aria-invalid', 'false');
        }
    });

    // Form submission handler
    form.addEventListener('submit', function(event) {
        event.preventDefault();

        // Validate all fields
        const usernameErr = validateUsername(usernameInput.value);
        const emailErr = validateEmail(emailInput.value);
        const passwordErr = validatePassword(passwordInput.value);
        const confirmPasswordErr = validateConfirmPassword(passwordInput.value, confirmPasswordInput.value);
        const termsErr = validateTerms(termsCheckbox.checked);

        // Display errors
        showError(usernameError, usernameErr);
        showError(emailError, emailErr);
        showError(passwordError, passwordErr);
        showError(confirmPasswordError, confirmPasswordErr);
        showError(termsError, termsErr);

        // Set aria-invalid attributes
        usernameInput.setAttribute('aria-invalid', usernameErr ? 'true' : 'false');
        emailInput.setAttribute('aria-invalid', emailErr ? 'true' : 'false');
        passwordInput.setAttribute('aria-invalid', passwordErr ? 'true' : 'false');
        confirmPasswordInput.setAttribute('aria-invalid', confirmPasswordErr ? 'true' : 'false');
        termsCheckbox.setAttribute('aria-invalid', termsErr ? 'true' : 'false');

        // Check if there are any errors
        if (usernameErr || emailErr || passwordErr || confirmPasswordErr || termsErr) {
            // Focus on first error field
            if (usernameErr) {
                usernameInput.focus();
            } else if (emailErr) {
                emailInput.focus();
            } else if (passwordErr) {
                passwordInput.focus();
            } else if (confirmPasswordErr) {
                confirmPasswordInput.focus();
            } else if (termsErr) {
                termsCheckbox.focus();
            }
            return;
        }

        // If validation passes, simulate successful registration
        handleSuccessfulRegistration();
    });

    function handleSuccessfulRegistration() {
        // Get form data
        const formData = {
            username: usernameInput.value,
            email: emailInput.value,
            password: passwordInput.value,
            accessibility: {
                highContrast: document.getElementById('highContrast').checked,
                screenReader: document.getElementById('screenReader').checked,
                reduceMotion: document.getElementById('reduceMotion').checked
            },
            timestamp: new Date().toISOString()
        };

        // Log registration data for debugging (password excluded for security)
        console.log('Registration submitted:', {
            username: formData.username,
            email: formData.email,
            accessibility: formData.accessibility,
            timestamp: formData.timestamp
        });

        // Hide form and show success message
        form.classList.add('hidden');
        successMessage.classList.remove('hidden');

        // Announce to screen readers
        successMessage.focus();

        // In a real application, you would send this data to a server
        // Example:
        // fetch('/api/register', {
        //     method: 'POST',
        //     headers: { 'Content-Type': 'application/json' },
        //     body: JSON.stringify(formData)
        // })
        // .then(response => response.json())
        // .then(data => {
        //     // Handle server response
        // })
        // .catch(error => {
        //     // Handle error
        // });
    }

    // Form reset handler
    form.addEventListener('reset', function() {
        // Clear all error messages
        clearError(usernameError);
        clearError(emailError);
        clearError(passwordError);
        clearError(confirmPasswordError);
        clearError(termsError);

        // Clear aria-invalid attributes
        usernameInput.removeAttribute('aria-invalid');
        emailInput.removeAttribute('aria-invalid');
        passwordInput.removeAttribute('aria-invalid');
        confirmPasswordInput.removeAttribute('aria-invalid');
        termsCheckbox.removeAttribute('aria-invalid');

        // Focus on first field after reset completes
        // setTimeout ensures focus happens after the form reset event has fully completed
        setTimeout(function() {
            usernameInput.focus();
        }, 0);
    });

    // Initialize: set aria-hidden on empty error messages
    const errorElements = [usernameError, emailError, passwordError, confirmPasswordError, termsError];
    errorElements.forEach(function(element) {
        element.setAttribute('aria-hidden', 'true');
    });

    // Focus on username field on page load
    window.addEventListener('load', function() {
        usernameInput.focus();
    });
})();
