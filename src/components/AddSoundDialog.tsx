import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Plus } from "lucide-react";
import { LANGUAGES, type Lang, type Sound } from "@/types";

export function AddSoundDialog({
  onCreated,
}: {
  onCreated: (sound: Sound) => void;
}) {
  const [open, setOpen] = useState(false);
  const [label, setLabel] = useState("");
  const [text, setText] = useState("");
  const [lang, setLang] = useState<Lang>("pt");
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  function reset() {
    setLabel("");
    setText("");
    setLang("pt");
    setError(null);
  }

  async function handleSave() {
    setSaving(true);
    setError(null);
    try {
      const sound = await invoke<Sound>("create_sound", { label, text, lang });
      onCreated(sound);
      reset();
      setOpen(false);
    } catch (err) {
      setError(String(err));
    } finally {
      setSaving(false);
    }
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(next) => {
        setOpen(next);
        if (!next) reset();
      }}
    >
      <DialogTrigger
        render={<Button className="gap-2 font-mono tracking-wide uppercase" />}
      >
        <Plus className="size-4" />
        New sound
      </DialogTrigger>
      <DialogContent className="font-mono sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="tracking-wide uppercase">
            Record new clip
          </DialogTitle>
          <DialogDescription>
            Type what the dictaphone should say. It'll be synthesized in a flat,
            robotic voice.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-2">
          <div className="grid gap-2">
            <Label htmlFor="sound-label">Label</Label>
            <Input
              id="sound-label"
              value={label}
              maxLength={40}
              placeholder="e.g. Intro"
              onChange={(e) => setLabel(e.target.value)}
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="sound-text">Text</Label>
            <Textarea
              id="sound-text"
              value={text}
              rows={4}
              placeholder="What should it say?"
              onChange={(e) => setText(e.target.value)}
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="sound-lang">Language</Label>
            <Select value={lang} onValueChange={(v) => setLang(v as Lang)}>
              <SelectTrigger id="sound-lang" className="w-full">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {LANGUAGES.map(({ value, label: langLabel }) => (
                  <SelectItem key={value} value={value}>
                    {langLabel}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          {error && (
            <p className="text-destructive text-sm" role="alert">
              {error}
            </p>
          )}
        </div>

        <DialogFooter>
          <Button
            onClick={handleSave}
            disabled={saving || !label.trim() || !text.trim()}
          >
            {saving ? "Recording…" : "Save"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
