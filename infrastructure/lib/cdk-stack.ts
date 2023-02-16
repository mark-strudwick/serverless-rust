import * as path from 'path';
import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'cargo-lambda-cdk';
import {HttpApi, HttpMethod} from '@aws-cdk/aws-apigatewayv2-alpha';
import {HttpLambdaIntegration} from '@aws-cdk/aws-apigatewayv2-integrations-alpha';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const func = new RustFunction(this, 'rust-function', {
      manifestPath: path.join(__dirname, '..', '..'),
      functionName: 'rust-function',
      binaryName: 'backend'
    });

    const apigw = new HttpApi(this, 'apigw', {
      apiName: 'serverless-rust',
    });

    apigw.addRoutes({
      path: '/',
      methods: [HttpMethod.ANY],
      integration: new HttpLambdaIntegration(
        'function-integration',
        func
      ), 
    });

    new cdk.CfnOutput(this, 'function name', {
      value: func.functionName,
    });
    new cdk.CfnOutput(this, 'api endpoint', {
      value: apigw.apiEndpoint,
    });
  }
}