// Example JavaScript file for testing the parser
function greet(name) {
    return `Hello, ${name}!`;
}

const message = greet("World");
console.log(message);

// Example with modules
export default greet;
export { message };