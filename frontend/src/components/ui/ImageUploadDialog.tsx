import {
  Dialog,
  DialogTrigger,
  DialogSurface,
  DialogTitle,
  DialogBody,
  DialogActions,
  DialogContent,
  Button,
  Input,
  Label,
  Spinner,
  Tab,
  TabList,
  makeStyles,
  tokens,
  shorthands,
  mergeClasses,
  Text,
} from '@fluentui/react-components';
import { useState, useRef, useEffect } from 'react';
import { filesApi } from '../../api';
import {
  ImageRegular,
  CloudRegular,
  ArrowUploadRegular,
  DocumentAddRegular,
  DismissRegular,
} from '@fluentui/react-icons';
import type { FileInfo } from '../../types';

const useStyles = makeStyles({
  dialogSurface: {
    maxWidth: '640px',
    width: '100%',
    height: 'fit-content',
    maxHeight: '90vh',
    display: 'flex',
    flexDirection: 'column',
  },
  content: {
    display: 'flex',
    flexDirection: 'column',
    gap: tokens.spacingVerticalL,
    paddingTop: tokens.spacingVerticalM,
    flexGrow: 1,
    overflowY: 'hidden', // Let children handle scroll
  },
  tabList: {
    ...shorthands.borderBottom('1px', 'solid', tokens.colorNeutralStroke2),
    paddingBottom: tokens.spacingVerticalS,
  },
  tabContent: {
    flexGrow: 1,
    overflowY: 'auto',
    paddingRight: tokens.spacingHorizontalS, // space for scrollbar
    minHeight: '300px',
    display: 'flex',
    flexDirection: 'column',
  },
  // Library Styles
  fileGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(140px, 1fr))',
    gap: tokens.spacingHorizontalM,
    paddingBottom: tokens.spacingVerticalM,
  },
  fileItem: {
    position: 'relative',
    cursor: 'pointer',
    ...shorthands.border('1px', 'solid', tokens.colorNeutralStroke1),
    ...shorthands.borderRadius(tokens.borderRadiusMedium),
    overflow: 'hidden',
    transition: 'all 0.2s cubic-bezier(0.33, 0, 0.67, 1)', // fast out, slow in
    display: 'flex',
    flexDirection: 'column',
    backgroundColor: tokens.colorNeutralBackground1,
    ':hover': {
      ...shorthands.borderColor(tokens.colorBrandStroke1),
      boxShadow: tokens.shadow4,
    },
  },
  fileItemSelected: {
    ...shorthands.borderColor(tokens.colorBrandStroke1),
    ...shorthands.borderWidth('2px'),
    backgroundColor: tokens.colorBrandBackground2,
    boxShadow: tokens.shadow8,
  },
  fileItemImage: {
    width: '100%',
    aspectRatio: '16/9',
    objectFit: 'cover',
    backgroundColor: tokens.colorNeutralBackground2,
  },
  fileItemName: {
    padding: tokens.spacingHorizontalS,
    fontSize: tokens.fontSizeBase200,
    color: tokens.colorNeutralForeground1,
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    whiteSpace: 'nowrap',
  },
  // Upload Styles
  uploadArea: {
    display: 'flex',
    flexDirection: 'column',
    gap: tokens.spacingVerticalL,
    alignItems: 'stretch',
  },
  dropZone: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    gap: tokens.spacingVerticalM,
    padding: tokens.spacingVerticalXXL,
    ...shorthands.border('2px', 'dashed', tokens.colorNeutralStroke1),
    ...shorthands.borderRadius(tokens.borderRadiusLarge),
    backgroundColor: tokens.colorNeutralBackgroundAlpha,
    cursor: 'pointer',
    transition: 'background-color 0.2s',
    ':hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
      ...shorthands.borderColor(tokens.colorBrandStroke1),
    },
  },
  dropZoneActive: {
    ...shorthands.borderColor(tokens.colorBrandStroke1),
    backgroundColor: tokens.colorBrandBackground2,
  },
  previewContainer: {
    position: 'relative',
    width: '100%',
    backgroundColor: tokens.colorNeutralBackground2,
    ...shorthands.borderRadius(tokens.borderRadiusMedium),
    ...shorthands.overflow('hidden'),
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    minHeight: '200px',
  },
  preview: {
    maxWidth: '100%',
    maxHeight: '300px',
    objectFit: 'contain',
  },
  removeBtn: {
    position: 'absolute',
    top: tokens.spacingVerticalS,
    right: tokens.spacingHorizontalS,
  },
  formField: {
    display: 'flex',
    flexDirection: 'column',
    gap: tokens.spacingVerticalS,
  },
  // Empty State
  emptyState: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    padding: tokens.spacingVerticalXXL,
    color: tokens.colorNeutralForeground3,
    gap: tokens.spacingVerticalM,
    height: '100%',
    minHeight: '200px',
  },
});

interface ImageUploadDialogProps {
  onInsert: (url: string, alt: string) => void;
}

export const ImageUploadDialog: React.FC<ImageUploadDialogProps> = ({ onInsert }) => {
  const styles = useStyles();
  const [open, setOpen] = useState(false);
  const [tabValue, setTabValue] = useState<'library' | 'upload'>('library');

  // File library state
  const [files, setFiles] = useState<FileInfo[]>([]);
  const [loadingFiles, setLoadingFiles] = useState(false);
  const [selectedFile, setSelectedFile] = useState<FileInfo | null>(null);

  // Upload state
  const [newFile, setNewFile] = useState<File | null>(null);
  const [preview, setPreview] = useState<string | null>(null);
  const [altText, setAltText] = useState('');
  const [uploading, setUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (open && tabValue === 'library') {
      void loadFiles();
    }
  }, [open, tabValue]);

  const loadFiles = async () => {
    try {
      setLoadingFiles(true);
      const response = await filesApi.getFiles({ per_page: 50 });
      // Filter only images
      const imageFiles = response.data.filter((f: FileInfo) => f.content_type.startsWith('image/'));
      setFiles(imageFiles);
    } catch (error) {
      console.error('Failed to load files:', error);
    } finally {
      setLoadingFiles(false);
    }
  };

  const handleFileSelect = (file: FileInfo) => {
    // Toggle selection
    if (selectedFile?.id === file.id) {
      setSelectedFile(null);
      setAltText('');
    } else {
      setSelectedFile(file);
      setAltText(file.original_filename.split('.')[0]);
    }
  };

  const processFile = (file: File) => {
    setNewFile(file);
    const objectUrl = URL.createObjectURL(file);
    setPreview(objectUrl);
    if (!altText) {
      const nameWithoutExt = file.name.split('.').slice(0, -1).join('.');
      setAltText(nameWithoutExt);
    }
  };

  const handleNewFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files?.[0]) {
      processFile(e.target.files[0]);
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    if (!isDragging) setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    if (e.relatedTarget && e.currentTarget.contains(e.relatedTarget as Node)) {
      return;
    }
    setIsDragging(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);

    const file = e.dataTransfer.files?.[0];
    if (file && file.type.startsWith('image/')) {
      processFile(file);
    }
  };

  const clearUpload = () => {
    setNewFile(null);
    setPreview(null);
    setAltText('');
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const handleUpload = async () => {
    if (!newFile) return;

    try {
      setUploadError(null);
      setUploading(true);
      const response = await filesApi.uploadFile({ file: newFile });
      handleInsert(response.url);
    } catch (error) {
      console.error('Upload failed:', error);
      setUploadError('上传失败，请重试');
    } finally {
      setUploading(false);
    }
  };

  const handleInsert = (url: string) => {
    onInsert(url, altText || 'image');
    handleClose();
  };

  const handleClose = () => {
    setOpen(false);
    // Reset state after dialog closes animation
    setTimeout(() => {
      setSelectedFile(null);
      clearUpload();
      setFiles([]);
      setTabValue('library');
    }, 200);
  };

  return (
    <Dialog open={open} onOpenChange={(_, data) => setOpen(data.open)}>
      <DialogTrigger disableButtonEnhancement>
        <Button icon={<ImageRegular />} size="small">
          插入图片
        </Button>
      </DialogTrigger>
      <DialogSurface className={styles.dialogSurface} aria-label="Insert Image Dialog">
        <DialogBody style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
          <DialogTitle>选择或上传图片</DialogTitle>
          <DialogContent className={styles.content}>
            <TabList
              selectedValue={tabValue}
              onTabSelect={(_, data) => setTabValue(data.value as 'library' | 'upload')}
              className={styles.tabList}
            >
              <Tab value="library" icon={<CloudRegular />}>
                文件库
              </Tab>
              <Tab value="upload" icon={<ArrowUploadRegular />}>
                上传新图
              </Tab>
            </TabList>

            <div className={styles.tabContent}>
              {tabValue === 'library' &&
                (loadingFiles ? (
                  <div className={styles.emptyState}>
                    <Spinner label="加载中..." />
                  </div>
                ) : files.length === 0 ? (
                  <div className={styles.emptyState}>
                    <CloudRegular fontSize="48px" />
                    <Text weight="semibold">暂无图片</Text>
                    <Text size={200}>上传第一张图片到"上传新图"标签页</Text>
                  </div>
                ) : (
                  <div className={styles.fileGrid}>
                    {files.map((file) => (
                      <div
                        key={file.id}
                        className={mergeClasses(
                          styles.fileItem,
                          selectedFile?.id === file.id && styles.fileItemSelected
                        )}
                        onClick={() => handleFileSelect(file)}
                        role="button"
                        aria-pressed={selectedFile?.id === file.id}
                        tabIndex={0}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter' || e.key === ' ') {
                            handleFileSelect(file);
                          }
                        }}
                      >
                        <img
                          src={file.url}
                          alt={file.original_filename}
                          className={styles.fileItemImage}
                        />
                        <div className={styles.fileItemName} title={file.original_filename}>
                          {file.original_filename}
                        </div>
                      </div>
                    ))}
                  </div>
                ))}

              {tabValue === 'upload' && (
                <div className={styles.uploadArea}>
                  {!preview ? (
                    <>
                      <input
                        type="file"
                        accept="image/*"
                        ref={fileInputRef}
                        onChange={handleNewFileChange}
                        style={{ display: 'none' }}
                        id="image-upload-input"
                      />
                      <label
                        htmlFor="image-upload-input"
                        onDragOver={handleDragOver}
                        onDragLeave={handleDragLeave}
                        onDrop={handleDrop}
                      >
                        <div
                          className={mergeClasses(
                            styles.dropZone,
                            isDragging && styles.dropZoneActive
                          )}
                          role="button"
                          tabIndex={0}
                        >
                          <DocumentAddRegular
                            fontSize="48px"
                            color={tokens.colorBrandForeground1}
                          />
                          <div style={{ textAlign: 'center' }}>
                            <Text weight="semibold" block>
                              {isDragging ? '松开以添加图片' : '点击或拖拽上传图片'}
                            </Text>
                            <Text size={200} style={{ color: tokens.colorNeutralForeground3 }}>
                              支持 JPG, PNG, GIF, WebP
                            </Text>
                          </div>
                          <Button appearance="secondary">浏览文件</Button>
                        </div>
                      </label>
                    </>
                  ) : (
                    <div className={styles.previewContainer}>
                      <img src={preview} alt="Preview" className={styles.preview} />
                      <Button
                        icon={<DismissRegular />}
                        appearance="subtle"
                        className={styles.removeBtn}
                        onClick={clearUpload}
                        title="移除图片"
                        aria-label="Remove selected image"
                      />
                    </div>
                  )}

                  <div className={styles.formField}>
                    <Label htmlFor="alt-text" weight="semibold">
                      图片描述 (Alt Text)
                    </Label>
                    <Input
                      id="alt-text"
                      value={altText}
                      onChange={(_, data) => setAltText(data.value)}
                      placeholder="描述图片内容，用于SEO和可访问性..."
                    />
                    <Text size={200} style={{ color: tokens.colorNeutralForeground3 }}>
                      良好的描述有助于屏幕阅读器用户理解图片内容。
                    </Text>
                  </div>

                  {uploadError && (
                    <Text size={200} style={{ color: tokens.colorPaletteRedForeground1 }}>
                      {uploadError}
                    </Text>
                  )}
                </div>
              )}
            </div>
          </DialogContent>
          <DialogActions>
            <Button appearance="secondary" onClick={handleClose}>
              取消
            </Button>
            {tabValue === 'library' ? (
              <Button
                appearance="primary"
                onClick={() => selectedFile && handleInsert(selectedFile.url)}
                disabled={!selectedFile}
              >
                插入选中的图片
              </Button>
            ) : (
              <Button
                appearance="primary"
                onClick={() => {
                  void handleUpload();
                }}
                disabled={!newFile || uploading}
              >
                {uploading ? <Spinner size="tiny" /> : '上传并插入'}
              </Button>
            )}
          </DialogActions>
        </DialogBody>
      </DialogSurface>
    </Dialog>
  );
};
