import {save_password, find_password} from '../../password_manager'

// Password Generator
function generatePassword() {
  const letters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
  const numbers = '0123456789';
  const symbols = '!#$%&()*+';

  const generateRandomChar = (characters) => characters[Math.floor(Math.random() * characters.length)];

  const nrLetters = Math.floor(Math.random() * 3) + 8;
  const nrNumbers = Math.floor(Math.random() * 3) + 2;
  const nrSymbols = Math.floor(Math.random() * 3) + 2;

  let password = '';

  for (let i = 0; i < nrLetters; i++) {
    password += generateRandomChar(letters);
  }

  for (let i = 0; i < nrNumbers; i++) {
    password += generateRandomChar(numbers);
  }

  for (let i = 0; i < nrSymbols; i++) {
    password += generateRandomChar(symbols);
  }

  password = password.split('').sort(() => Math.random() - 0.5).join('');

  document.getElementById('password').value = password;
}




// Save Password
function savePassword() {
  const website = document.getElementById('website').value;
  const email = document.getElementById('email').value;
  const password = document.getElementById('password').value;

  if (!website || !password) {
    alert('Please make sure you haven\'t left any fields empty.');
    return;
  }

  const isOk = confirm(`These are the details entered:\nWebsite: ${website}\nEmail/UserName: ${email}\nPassword: ${password}\nIs it OK to save?`);

  if (isOk) {
    // Call the save_password function from Rust
    save_password(website, email, password);
    alert('Password Saved.');
  }
}

// Find Password
function findPassword() {
  const website = document.getElementById('website').value;

  if (!website) {
    alert('Please enter a website.');
    return;
  }

  // Call the find_password function from Rust
  const passwordData = find_password(website);

  if (passwordData !== null) {
    // Update the following lines based on how you want to display the retrieved data
    const email = passwordData.email_or_username;
    const password = passwordData.password;

    document.getElementById('email').value = email;
    document.getElementById('password').value = password;
  } else {
    alert(`No details found for the specified website: ${website}`);
  }
}
