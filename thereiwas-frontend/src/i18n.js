import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {
      en: {
        translation: {
          login: {
            headlines: {
              signin: "Sign in",
            },
            form: {
              username_field_label: "Username",
              password_field_label: "Password",
              sign_in_button: "Sign in",
            },
            alerts: {
              failed_login: {
                message: "Failed to sign in. Please try again.",
              },
            },
          },
          dashboard: {
            navigation: {
              app_title: "There I Was",
              logout_button: "Logout",
            },
          },
          global: {
            navigation: {
              dashboard: "Dashboard",
              calendar: "Calendar",
              settings: "Settings",
              version: "Version",
            },
          },
        },
      },
    },
    fallbackLng: "en",
    interpolation: {
      escapeValue: false, // React already escapes the values
    },
  });

export default i18n;
