import os
import subprocess
import json

def lambda_handler(event, context):
    try:
        github_token = event.get('github_token', '')
        repo_url = event.get('repo_url', '')

        if not github_token:
            return {
                'statusCode': 400,
                'body': 'GitHub token is missing.'
            }

        if not repo_url:
            return {
                'statusCode': 400,
                'body': 'Repository url is missing.'
            }

        runner_dir = "/actions-runner"

        os.chdir(runner_dir)

        env_vars = os.environ.copy()
        env_vars["RUNNER_ALLOW_RUNASROOT"] = "1"

        config_result = subprocess.run(
            [
                "./config.sh",
                "--url", repo_url,
                "--token", github_token,
                "--ephemeral",
                "--unattended",
                "--labels", "ephemeral-lambda",
                "--disableupdate"
            ],
            capture_output=True,
            env=env_vars
        )

        if config_result.returncode != 0:
            return {
                'statusCode': 500,
                'body': f"Failed to configure runner: {config_result.stderr.decode('utf-8')}"
            }

        run_result = subprocess.run(
            [
                "./run.sh",
            ],
            capture_output=True,
            env=env_vars
        )

        if run_result.returncode == 0:
            return {
                'statusCode': 200,
                'body': 'Successfully executed GitHub Actions runner.'
            }
        else:
            return {
                'statusCode': 500,
                'body': f"Failed to start runner: {run_result.stderr.decode('utf-8')}"
            }

    except Exception as e:
        return {
            'statusCode': 500,
            'body': f"Exception: {str(e)}"
        }
