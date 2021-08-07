import React from "react";
import { makeStyles } from "@material-ui/styles";
import { Button, Typography, TextField } from "@material-ui/core";
import { Redirect } from "react-router";
import { View } from "views/view";

import { PropsFromConnector } from ".";
import { Form, Formik } from "formik";

const useStyles = makeStyles(() => ({
  root: {
    justifyContent: "center",
  },
  title: {
    marginBottom: "32px",
  },
  textField: {
    marginBottom: "32px",
  },
  form: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
  },
}));

interface IFormikValues {
  login: string;
  password: string;
}

const SignIn: React.FC<PropsFromConnector> = ({
  isAuthorized,
  actionSignIn,
}: PropsFromConnector) => {
  const classes = useStyles();

  const formikValidation = (values: IFormikValues) => {
    const errors: { login?: string; password?: string } = {};
    if (values.login === "") {
      errors.login = "Login must not be empty!";
    }
    if (values.password === "") {
      errors.password = "Password must not be empty!";
    }
    return errors;
  };

  if (isAuthorized) {
    return <Redirect to="/" />;
  }

  return (
    <View className={classes.root}>
      <Formik
        initialValues={{
          login: "",
          password: "",
        }}
        validate={formikValidation}
        onSubmit={(values) => {
          actionSignIn(values.login, values.password);
        }}
      >
        {(formik) => (
          <Form className={classes.form}>
            <Typography variant="h6" className={classes.title}>
              Sign in
            </Typography>
            <TextField
              autoFocus={true}
              className={classes.textField}
              autoComplete="off"
              id="login"
              name="login"
              label="Login"
              value={formik.values.login}
              onChange={formik.handleChange}
              error={Boolean(formik.errors.login)}
              helperText={formik.errors.login}
            />
            <TextField
              className={classes.textField}
              autoComplete="off"
              id="password"
              name="password"
              label="Password"
              value={formik.values.password}
              onChange={formik.handleChange}
              error={Boolean(formik.errors.password)}
              helperText={formik.errors.password}
            />
            <Button variant="contained" color="primary" type="submit">
              Login
            </Button>
          </Form>
        )}
      </Formik>
    </View>
  );
};

export { SignIn };
