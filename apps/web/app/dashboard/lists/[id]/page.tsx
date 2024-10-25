"use client";

import { useCreateTodo } from "@/api/mutation/useCreateTodo";
import { useUpdateTodo } from "@/api/mutation/useUpdateTodo";
import { useFetchList } from "@/api/query/useFetchList";
import { Container } from "@/components/Container";
import { Loading } from "@/components/Loading";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardTitle,
} from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { useModal } from "@/hooks/useModal";
import { cn } from "@/lib/utils";
import { dayjs } from "@/utils/dayjs";
import { toastError } from "@/utils/toast";
import { FC, FormEvent, useRef, useState } from "react";
import { GetTodoListDetailsResponse } from "server";
import { toast } from "sonner";

export default function ListPage({ params }: { params: { id: string } }) {
  const { data } = useFetchList(params.id);

  if (!data) return <Loading />;

  return (
    <Container>
      <div>
        <p className="text-xl capitalize">{data.list.name}</p>
        <small className="text-slate-500">
          Created {dayjs(data.list.created_at).fromNow()}
        </small>
      </div>

      <AddTodoButton listId={params.id} className="my-10" />

      <div className="flex flex-col gap-2">
        {data.items.map((todo) => (
          <TodoItem key={todo.id} todo={todo} />
        ))}
      </div>

      {data.items.length === 0 && (
        <div className="flex justify-center items-center h-32">
          <p className="text-2xl text-slate-500">No todos found</p>
        </div>
      )}
    </Container>
  );
}

export const TodoItem: FC<{
  todo: GetTodoListDetailsResponse["items"][number];
}> = ({ todo }) => {
  const [checked, setChecked] = useState(todo.completed);

  const { mutateAsync } = useUpdateTodo(todo.id);

  const onChange = (completed: boolean) => {
    mutateAsync({ completed })
      .then(() => {
        toast.success("Todo updated successfully");
        setChecked(completed);
      })
      .catch(toastError);
  };

  return (
    <Card className="flex flex-col justify-between lg:flex-row lg:items-center gap-2 p-3">
      <CardContent>
        <CardTitle>{todo.title}</CardTitle>
        <CardDescription className="mt-3">{todo.description}</CardDescription>

        <small className="text-gray-500">
          Created {dayjs(todo.created_at).fromNow()}
        </small>
      </CardContent>

      <div className="flex items-center gap-3">
        <p
          className={cn({
            "text-gray-500": !checked,
            "text-green-500": checked,
          })}
        >
          {checked ? "Completed" : "Not Completed"}{" "}
        </p>
        <Switch onCheckedChange={onChange} checked={checked} />
      </div>
    </Card>
  );
};

export const AddTodoButton: FC<{ listId: string; className?: string }> = ({
  className,
  listId,
}) => {
  const titleRef = useRef<HTMLInputElement>(null);
  const descriptionRef = useRef<HTMLInputElement>(null);

  const { close, isOpen, open, setIsOpen } = useModal();

  const { mutateAsync } = useCreateTodo();
  const { refetch } = useFetchList(listId);

  const onSubmit = (e: FormEvent) => {
    e.preventDefault();

    const title = titleRef.current?.value;
    const description = descriptionRef.current?.value;

    mutateAsync({
      title,
      description,
      list_id: listId,
    })
      .then(() => {
        toast.success("Todo created successfully");
        refetch();
        close();
      })
      .catch(toastError);
  };

  return (
    <div className={cn("", className)}>
      <Button onClick={open}>Add Todo</Button>

      <Dialog open={isOpen} onOpenChange={setIsOpen}>
        <DialogContent>
          <form onSubmit={onSubmit}>
            <DialogHeader>
              <DialogTitle>Add a new todo item</DialogTitle>
            </DialogHeader>

            <div className="flex flex-col gap-2 mt-4">
              <Input ref={titleRef} placeholder="Title" />
              <Input ref={descriptionRef} placeholder="Description" />
            </div>

            <DialogFooter className="mt-2">
              <Button>Add</Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
};
