import { createTheme } from "@material-ui/core/styles";

const darkTheme = createTheme({
  overrides: {
    MuiInputLabel: {
      root: {
        color: "white",
        "&$focused": {
          color: "#33fff8",
        },
      },
    },
    MuiSelect: {
      icon: {
        color: "white",
      },
    },
    MuiInput: {
      root: {
        color: "white",
      },
      underline: {
        "&:before": {
          borderBottom: `2px solid white`,
        },
        "&:after": {
          borderBottom: `2px solid #33fff8`,
        },
      },
    },
  },
  palette: {
    primary: {
      main: "#0F0000",
    },
    common: {
      black: "#FF0000",
    },
    text: {
      primary: "#FFFFFF",
      secondary: "#EEEEEE",
      hint: "#FF0000",
    },
    action: {
      focus: "#00FFFF",
    },
    background: {
      paper: "#555555",
      default: "#222222",
    },
  },
});

export { darkTheme };
