import * as path from 'path';
import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'cargo-lambda-cdk';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const func = new RustFunction(this, 'rust-function', {
      manifestPath: path.join(__dirname, '..', '..')
    });

    new cdk.CfnOutput(this, 'function name', {
      value: func.functionName,
    });
  }
}