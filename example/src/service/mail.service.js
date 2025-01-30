
import dotenv from 'dotenv';  // Import dotenv to load environment variables
import mailgun from 'mailgun-js';  // Import mailgun-js for sending emails

// Load environment variables from .env file
dotenv.config();

// Retrieve Mailgun configuration from environment variables
const apiKey = process.env.MAILGUN_API_KEY;
const domain = process.env.MAILGUN_DOMAIN;
const from = process.env.MAILGUN_FROM_EMAIL;  // Sender's email


// Initialize Mailgun with the API key and domain
const mg = mailgun({ apiKey, domain });

// Function to send email with HTML content and a reply-to address
export const sendEmail = (email, subject, text, htmlContent, replyTo) => {
  const data = {
    from,
    to: email,
    subject,
    text,  // Plain text version
    html: htmlContent,  // HTML content for the email
    'h:Reply-To': replyTo // Reply-To header
  };

  // Send the email using Mailgun API
  mg.messages().send(data, (error, body) => {
    if (error) {
      console.error('Error:', error);  // Log any errors if the email fails
    } else {
      console.log('Email sent successfully:', body);  // Log success if email is sent
    }
  });
};
