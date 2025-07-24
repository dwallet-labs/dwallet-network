#!/bin/bash

# Parallel Performance Test Runner
# Usage: ./run-parallel-tests.sh [parallel_count] [timeout_ms]

set -e

# Default values
DEFAULT_PARALLEL_COUNT=8
DEFAULT_TIMEOUT=600000  # 10 minutes

# Parse arguments
PARALLEL_COUNT=${1:-$DEFAULT_PARALLEL_COUNT}
TIMEOUT=${2:-$DEFAULT_TIMEOUT}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 dWallet MPC Parallel Performance Tests${NC}"
echo -e "${BLUE}=========================================${NC}"
echo -e "Parallel Count: ${GREEN}${PARALLEL_COUNT}${NC}"
echo -e "Timeout: ${GREEN}${TIMEOUT}ms${NC} ($(($TIMEOUT / 1000))s)"
echo ""

# Check if we're in the right directory
if [ ! -f "parallel-performance.test.ts" ]; then
    echo -e "${RED}❌ Error: parallel-performance.test.ts not found${NC}"
    echo -e "${YELLOW}Please run this script from the sdk/typescript/test/e2e directory${NC}"
    exit 1
fi

# Export environment variables
export PARALLEL_COUNT=$PARALLEL_COUNT
export TEST_TIMEOUT=$TIMEOUT

echo -e "${YELLOW}⏳ Starting parallel performance tests...${NC}"
echo ""

# Run the tests
if npm test parallel-performance.test.ts; then
    echo ""
    echo -e "${GREEN}✅ All parallel performance tests completed successfully!${NC}"
else
    echo ""
    echo -e "${RED}❌ Some tests failed. Check the output above for details.${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}📊 Test Summary:${NC}"
echo -e "   Configuration: ${PARALLEL_COUNT} parallel operations"
echo -e "   Timeout: ${TIMEOUT}ms per test"
echo -e "   Check the detailed performance metrics above"
echo ""
echo -e "${YELLOW}💡 Tips:${NC}"
echo -e "   • Increase parallel count for load testing: ./run-parallel-tests.sh 16"
echo -e "   • Extend timeout for slower networks: ./run-parallel-tests.sh 8 900000"
echo -e "   • Quick test with fewer operations: ./run-parallel-tests.sh 2" 