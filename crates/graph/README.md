### axiston/graph

Lorem Ipsum. Lorem Ipsum. Lorem Ipsum.

#### Notes

- Lorem Ipsum.
- Lorem Ipsum.
- Lorem Ipsum.

#### Flow

#### Step I: Initial Communication and Data Loading

1. Client Initialization:
   - The client requests the initial state of `InputGraph` (either empty or
     previously submitted) with a version identifier (consists out of
     `InputNode`s and `InputEdge`s and includes all data related to the
     workflow).

2. Registry Data Transfer:
   - The server sends the initial `TaskRegistry` and `HookRegistry` (consist out
     of only essential `TaskRegistryChunk`s and`HookRegistryChunk`s and include
     all includes all available tasks and hooks).
   - Additional data is loaded asynchronously based on client requests.

#### Step II: Graph Submission and Transformation

1. Submission to Server:
   - The validated `InputGraph` is submitted to the server for transformation.

2. Transformation and Error Reporting:
   - The server converts the `InputGraph` into `ProcessGraph`, returning
     detailed error feedback including specific node/edge references and
     suggestions for resolution.

#### Step III: Workflow Execution and Real-Time Feedback

1. Execution Request:
   - The client can trigger execution (called `InputEvent`) or the server can
     initiate it based on preconfigured conditions, using a locking mechanism to
     manage conflicts.

2. Real-Time Monitoring:
   - The server executes the graph and streams real-time updates as an
     `OutputGraph` (consists out of `OutputNode`s and `OutputEdge`s and includes
     all data related to the execution) and error feedback to the client during
     execution.

3. On-Demand Output Retrieval:
   - The server initially sends high-level execution results; detailed output
     can be requested as needed.
