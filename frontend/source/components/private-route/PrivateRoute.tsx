import React from "react";
import { Redirect, Route, RouteProps } from "react-router";
import { SIGN_IN } from "constants/routes";
import { PropsFromConnector } from ".";

type IProps = PropsFromConnector & RouteProps;

const PrivateRoute: React.FC<RouteProps> = ({
  isAuthorized,
  ...routeProps
}: IProps) => {
  console.log("isAuth = ", isAuthorized);
  if (isAuthorized) {
    return <Route {...routeProps} />;
  } else {
    return <Redirect to={{ pathname: SIGN_IN }} />;
  }
};

export { PrivateRoute };
