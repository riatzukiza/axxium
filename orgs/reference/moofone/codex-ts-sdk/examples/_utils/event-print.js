function flattenContent(value) {
  const parts = [];

  const walk = (REDACTED_SECRET) => {
    if (!REDACTED_SECRET) {
      return;
    }
    if (typeof REDACTED_SECRET === 'string') {
      parts.push(REDACTED_SECRET);
      return;
    }
    if (typeof REDACTED_SECRET.text === 'string') {
      parts.push(REDACTED_SECRET.text);
    }
    if (Array.isArray(REDACTED_SECRET.content)) {
      for (const item of REDACTED_SECRET.content) {
        walk(item);
      }
    }
    if (REDACTED_SECRET.type === 'tool_call') {
      const name = REDACTED_SECRET.tool_name ?? 'tool';
      parts.push(`[tool_call:${name}]`);
    }
  };

  walk(value);
  return parts;
}

export function describeAgentMessage(event) {
  const message = event.msg.message;
  if (!message) {
    return undefined;
  }

  if (typeof message === 'string') {
    return message;
  }

  const parts = flattenContent(message);
  if (parts.length > 0) {
    return parts.join('\n');
  }

  try {
    return JSON.stringify(message, null, 2);
  } catch {
    return String(message);
  }
}

export function describeAgentMessageDelta(event) {
  const delta = event.msg.delta ?? event.msg.message ?? event.msg;
  if (!delta) {
    return undefined;
  }

  if (typeof delta === 'string') {
    return delta;
  }

  const parts = flattenContent(delta);
  if (parts.length > 0) {
    return parts.join('');
  }

  if (typeof delta.text === 'string') {
    return delta.text;
  }

  try {
    return JSON.stringify(delta);
  } catch {
    return String(delta);
  }
}

export function describeReasoning(event) {
  if (event.msg.type === 'agent_reasoning_delta') {
    return typeof event.msg.delta === 'string' ? event.msg.delta : undefined;
  }
  if (event.msg.type === 'agent_reasoning') {
    if (Array.isArray(event.msg.reasoning)) {
      return event.msg.reasoning
        .map((chunk) => (typeof chunk === 'string' ? chunk : chunk?.text))
        .filter(Boolean)
        .join('\n');
    }
  }
  return undefined;
}
