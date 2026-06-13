import { describe, it, expect } from "vitest";
import React from "react";
import { render, screen } from "@testing-library/react";
import ModalWrapper from "../components/common/ModalWrapper";

describe("ModalWrapper", () => {
  it("should not render when isOpen is false", () => {
    const { container } = render(
      <ModalWrapper isOpen={false} onClose={() => {}}>
        <p>content</p>
      </ModalWrapper>
    );
    expect(container.innerHTML).toBe("");
  });

  it("should render children when isOpen is true", () => {
    render(
      <ModalWrapper isOpen={true} onClose={() => {}}>
        <p>test content</p>
      </ModalWrapper>
    );
    expect(screen.getByText("test content")).toBeInTheDocument();
  });
});
