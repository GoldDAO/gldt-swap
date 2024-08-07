#!/usr/bin/env bash

show_help() {
  cat << EOF
token_metrics canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-icp-neuron.sh [options] <NETWORK>

Options:
  -h, --help        Show this message and exit
  -r, --reinstall   Completely reinstall the canister, instead of simply upgrade it
EOF
}

# TODO: add a --identity option ?? (See dfx deploy --identity)
if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      -r | --reinstall )
        REINSTALL="--mode reinstall"
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <NETWORK> argument"
  exit 1
fi

if [[ ! $1 =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi

if [[ $1 =~ ^(local|staging)$ ]]; then
  TESTMODE="true"
  SNS_REWARDS_CANISTER_ID=$(dfx canister id --network $1 sns_rewards)
else
  TESTMODE="false"
  SNS_REWARDS_CANISTER_ID=$(dfx canister id --ic sns_rewards)
fi

INIT_ARGS="(opt record {
  test_mode = $TESTMODE;
  sns_rewards_canister_id = principal \"$SNS_REWARDS_CANISTER_ID\"
  })"

if [[ $1 == "local" ]]; then
  dfx deploy token_metrics --network $1 ${REINSTALL} --argument "$INIT_ARGS"  -y
elif [[ $CI_COMMIT_REF_NAME == "develop" || ( $1 == "ic" && $CI_COMMIT_TAG =~ ^token_metrics-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ) ]]; then

  echo "Deploying token_metrics with args \n $INIT_ARGS"
  # This is for direct deployment via CICD identity
  dfx deploy token_metrics --network $1 ${REINSTALL} --argument "$INIT_ARGS" -y

  # The following lines are for deployment via SNS. Only activate when handing over the canister
  # TODO - make sure to improve this procedure, created issue #156 to address this

  # if [[ $1 == "ic" ]]; then
  #   PROPOSER=$SNS_PROPOSER_NEURON_ID_PRODUCTION
  #   UPGRADEVERSION=$CI_COMMIT_TAG
  # else
  #   PROPOSER=$SNS_PROPOSER_NEURON_ID_STAGING
  #   UPGRADEVERSION=$CI_COMMIT_SHORT_SHA
  # fi
  # . scripts/prepare_sns_canister_ids.sh $1 && \
  # . scripts/prepare_proposal_summary.sh token_metrics && \
  # quill sns --canister-ids-file sns_canister_ids.json make-upgrade-canister-proposal $PROPOSER \
  #   --pem-file $PEM_FILE \
  #   --canister-upgrade-arg '(opt record {test_mode = '$TESTMODE' })' \
  #   --target-canister-id $(cat canister_ids.json | jq -r .token_metrics.$1) \
  #   --wasm-path backend/canisters/token_metrics/target/wasm32-unknown-unknown/release/token_metrics_canister.wasm.gz \
  #   --title "Upgrade token_metrics to ${UPGRADEVERSION}" \
  #   --url ${DETAILS_URL} --summary-path proposal.md | quill send --yes -
fi
return
