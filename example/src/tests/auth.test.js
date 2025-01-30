// tests/auth.test.js
const authService = require('../src/service/auth');

describe('Authentication Service', () => {
  test('should return a token for valid credentials', () => {
    const result = authService.login('admin', 'password123');
    expect(result).toHaveProperty('token');
    expect(result.token).toBe('fake-jwt-token');
  });

  test('should throw an error for missing username or password', () => {
    expect(() => authService.login()).toThrow('Username and password are required');
  });

  test('should throw an error for invalid credentials', () => {
    expect(() => authService.login('user', 'wrongpassword')).toThrow('Invalid credentials');
  });
});
