import React from "react";
import { Redirect, Route, RouteProps } from "react-router";
import { SIGN_IN } from "constants/routes";

export type PrivateRouteProps = {
  isAuthenticated: boolean;
  authenticationPath: string;
} & RouteProps;

const PrivateRoute: React.FC<PrivateRouteProps> = ({
  isAuthenticated,
  authenticationPath,
  ...routeProps
}: PrivateRouteProps) => {
  console.log("Private route");
  if (isAuthenticated) {
    return <Route {...routeProps} />;
  } else {
    return <Redirect to={{ pathname: authenticationPath }} />;
  }
};

const HOC: React.FC<RouteProps> = (props: RouteProps) => {
  const isAuthenticated = false;
  return (
    <PrivateRoute
      {...props}
      isAuthenticated={isAuthenticated}
      authenticationPath={SIGN_IN}
    ></PrivateRoute>
  );
};

export { HOC as PrivateRoute };
