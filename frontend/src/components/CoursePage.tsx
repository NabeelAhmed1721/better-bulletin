import { useEffect, useRef, useState } from 'react';
import dagre from 'dagre';
import ReactFlow, { type Edge, type Node } from 'reactflow';

import 'reactflow/dist/style.css';
import { SearchResultStatus } from '../util';
import Header from './Header';

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));

const nodeWidth = 172;
const nodeHeight = 36;

const getLayoutedElements = (nodes, edges, direction = 'TB') => {
  const isHorizontal = direction === 'LR';
  dagreGraph.setGraph({ rankdir: direction });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  nodes.forEach((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    node.targetPosition = isHorizontal ? 'left' : 'top';
    node.sourcePosition = isHorizontal ? 'right' : 'bottom';

    // We are shifting the dagre node position (anchor=center center) to the top left
    // so it matches the React Flow node anchor point (top left).
    node.position = {
      x: nodeWithPosition.x - nodeWidth / 2,
      y: nodeWithPosition.y - nodeHeight / 2,
    };

    return node;
  });

  return { nodes, edges };
};

type CoursePageProps = {
  id: number; // unique course id
};

type RequirementTreeNode = {
  id: number;
  logic: string;
  course_id: number | null;
  req_course_id: number | null;
  parent: number | null;
};

type UndergraduateCourse = {
  id: number;
  code: string;
  number: number;
  suffix: string | null;

  title: string;
  description: string;
  credits: number;
  min_credits: number | null;

  GA: boolean; // Arts
  GHW: boolean; // Health and Wellness
  GH: boolean; // Humanities
  GN: boolean; // Natural Sciences
  GQ: boolean; // Quantification
  GS: boolean; // Social and Behavioral Sciences
  GWS: boolean; // Writing and Speaking

  ITD: boolean; // Inter-Domain
  LKD: boolean; // Linked

  FYS: boolean; // First-Year Seminar
  IC: boolean; // International Cultures
  US: boolean; // United States Cultures
  WCC: boolean; // Writing Across the Curriculum

  // B.A. Requirements
  BA: boolean; // Bachelor of Arts: Arts
  BH: boolean; // Bachelor of Arts: Humanities
  BN: boolean; // Bachelor of Arts: Natural Sciences
  BO: boolean; // Bachelor of Arts: Other Cultures
  BQ: boolean; // Bachelor of Arts: Quantification
  BS: boolean; // Bachelor of Arts: Social and Behavioral Sciences
  BF1: boolean; // Bachelor of Arts: Foreign/World Lang (12th Unit)
  BF2: boolean; // Bachelor of Arts: 2nd Foreign/World Language (All)

  // requirement trees
  crosslists: number[];
  prerequisites: RequirementTreeNode[];
  concurrent: RequirementTreeNode[];
  corequisites: RequirementTreeNode[];
  recommended: RequirementTreeNode[];
};

export default function CoursePage({ id }: CoursePageProps) {
  const reactflowRef = useRef<HTMLDivElement>(null);
  const [searchResults, setSearchResults] = useState<{
    data: UndergraduateCourse | undefined;
    status: SearchResultStatus;
  }>({ data: undefined, status: SearchResultStatus.Empty });
  const [loading, setLoading] = useState(false);
  const [flowNodes, setFlowNodes] = useState([]);
  const [flowEdges, setFlowEdges] = useState([]);

  useEffect(() => {
    Promise.all([
      fetch(`http://localhost:3001/api/course/${id}`).then((res) => res.json()),
      fetch(`http://localhost:3001/api/course/crosslists/${id}`).then((res) =>
        res.json(),
      ),
      fetch(`http://localhost:3001/api/course/prerequisites/${id}`).then(
        (res) => res.json(),
      ),
      fetch(`http://localhost:3001/api/course/concurrent/${id}`).then((res) =>
        res.json(),
      ),
      fetch(`http://localhost:3001/api/course/corequisites/${id}`).then((res) =>
        res.json(),
      ),
      fetch(`http://localhost:3001/api/course/recommended/${id}`).then((res) =>
        res.json(),
      ),
    ])
      .then((data) => {
        return setSearchResults({
          data: {
            ...data[0],
            crosslists: data[1].crosslists,
            prerequisites: data[2],
            concurrent: data[3],
            corequisites: data[4],
            recommended: data[5],
          },
          status: SearchResultStatus.Success,
        });
      })
      .catch(console.error);
  }, []);

  useEffect(() => {
    if (searchResults.status == SearchResultStatus.Success) {
      // start building nodes
      setLoading(true);

      createNodesFromTree()
        .then(({ nodes, edges }) => {
          setFlowEdges(edges);
          setFlowNodes(nodes);
        })
        .finally(() => {
          setLoading(false);
        })
        .catch(console.error);
    }
  }, [searchResults.status]);

  async function createNodesFromTree() {
    const nodes: Node[] = [
      {
        id: '1',
        data: {
          label: `${searchResults.data?.code} ${searchResults.data?.number}${searchResults.data?.suffix || ''}`,
        },
        // half of screen
        position: { x: 0, y: 0 },
      },
    ];

    const edges: Edges[] = [];
    const stack = [];
    const tree = searchResults.data?.concurrent || [];

    for (const i in tree) {
      const node = tree[i];

      nodes.push({
        id: node.id.toString(),
        data: {
          label: await (async () => {
            if (node.logic === '&') {
              if (stack.length === 0) {
                edges.push({
                  target: node.id.toString(),
                  source: '1',
                });
              }

              stack.push(node.id.toString());

              return 'AND';
            } else if (node.logic === '|') {
              if (stack.length === 0) {
                edges.push({
                  source: '1',
                  target: node.id.toString(),
                });
              }
              stack.push(node.id.toString());

              return 'OR';
            } else if (node.logic === 'C') {
              const data = await fetch(
                `http://localhost:3001/api/course/${node.req_course_id}`,
              )
                .then((res) => res.json())
                .then((data) => data);

              if (stack.length > 0) {
                edges.push({
                  source: stack[stack.length - 1],
                  target: node.id.toString(),
                });
              }

              return `${data.code} ${data.number}${data.suffix || ''}`;
            }
          })(),
        },
        position: { x: 0, y: nodes.length * 50 },
      });
    }

    return { nodes, edges };
  }

  if (searchResults.status === SearchResultStatus.Empty) {
    return (
      <div className="flex h-screen w-full items-center justify-center text-2xl font-bold text-psu-300">
        Loading...
      </div>
    );
  }

  const { nodes, edges } = getLayoutedElements(flowNodes, flowEdges, 'TB');

  return (
    <>
      <Header />
      <div className="w-full">
        <ReactFlow
          proOptions={{ hideAttribution: true }}
          fitView
          nodesConnectable={false}
          ref={reactflowRef}
          nodes={nodes}
          edges={edges}
        />
        {/* <p>{JSON.stringify(searchResults)}</p> */}
      </div>
    </>
  );
}
